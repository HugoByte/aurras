use super::*;
use kuska_ssb::api::dto::content::Mention;

/// This Rust code defines an implementation for a `Client` struct with various asynchronous
/// methods for interacting with a remote server using RPC (Remote Procedure Call) communication. Here
/// is a summary of what each method does:
impl Client {
    /// The `get_async` method in Rust is an asynchronous method that receives messages and processes
    /// them based on the request number and message type.
    ///
    /// # Arguments:
    ///
    /// * `req_no`: An instance of `RequestNo` that represents the request number that the method is waiting for.
    /// The method will keep looping and checking incoming messages until it receives a response with the
    /// matching request number.
    /// * `f`: A closure that takes a reference to a slice of `u8` bytes as input and returns a `Result<T>`,
    /// where `T` implements the `Debug` trait. This closure is used to process the body of the received
    ///
    /// # Returns:
    ///
    /// Returns a `Result<T>`, where `T` is the type specified in the method signature. The result can either
    /// be an `Ok` variant containing the value of type `T` if the operation is successful, or an `Err`
    /// variant containing an error if an error occurs during the execution of the method.
    async fn get_async<'a, T, F>(&mut self, req_no: RequestNo, f: F) -> Result<T>
    where
        F: Fn(&[u8]) -> Result<T>,
        T: Debug,
    {
        loop {
            let (id, msg) = self.rpc_reader.recv().await?;
            if id == req_no {
                match msg {
                    RecvMsg::RpcResponse(_type, body) => {
                        return f(&body);
                    }
                    RecvMsg::ErrorResponse(message) => {
                        return std::result::Result::Err(Box::new(AppError::new(message)));
                    }
                    _ => {}
                }
            }
        }
    }

    /// This Rust method asynchronously receives and processes responses based on a provided closure,
    /// handling different message types and error cases.
    ///
    /// # Arguments:
    ///
    /// * `req_no`: A `RequestNo` instance that represents the request number that is being processed. It is
    /// used to match the incoming messages with the corresponding request for processing.
    /// * `f`: A closure that takes a slice of bytes (`&[u8]`) as input and returns a result of type `T`.
    /// The closure is used to deserialize the byte slice into a value of type `T`.
    ///
    /// # Returns:
    ///
    /// Returns a `Result` containing a vector of type `T`.
    async fn get_responses<'a, T, F>(&mut self, req_no: RequestNo, f: F) -> Result<Vec<T>>
    where
        F: Fn(&[u8]) -> Result<T>,
        T: Debug + serde::Deserialize<'a>,
    {
        let mut response = vec![];

        loop {
            let (id, msg) = self.rpc_reader.recv().await?;

            if id == req_no {
                match msg {
                    RecvMsg::RpcResponse(_type, body) => {
                        let display = f(&body);

                        match display {
                            Ok(display) => response.push(display),
                            Err(err) => {
                                let body = match std::str::from_utf8(&body) {
                                    Ok(body_str) => body_str,
                                    Err(err) => {
                                        eprintln!(
                                            "Error converting response body to string: {}",
                                            err
                                        );
                                        continue;
                                    }
                                };

                                if body == "{\"sync\":true}" {
                                    println!("Syncing Successful");
                                } else {
                                    return std::result::Result::Err(err);
                                }
                            }
                        }
                    }
                    RecvMsg::ErrorResponse(message) => {
                        return std::result::Result::Err(Box::new(AppError::new(message)));
                    }
                    RecvMsg::CancelStreamResponse() => break,
                    _ => {}
                }
            }
        }

        Ok(response)
    }

    /// The method `ssb_handshake` establishes a secure Scuttlebutt handshake with a server using
    /// provided keys and connection details.
    ///
    /// # Arguments:
    ///
    /// * `pk`: A `PublicKey` instance representing the public key of the client.
    /// * `sk`: A `SecretKey` instance representing the secret key of the client. It is typically a
    /// cryptographic key that is kept secret and used for secure communication between the client
    /// and server. In this context, it is likely an Ed25519 secret key.
    /// * `id`: A `String` instance represents the identity of the server in the Secure Scuttlebutt(SSB)
    /// protocol. It is used to uniquely identify the server and establish a secure connection between
    /// the client and the server.
    /// * `ip`: A `String` instance represents the IP address of the server you want to connect to.
    /// It is a string that contains the IPv4 or IPv6 address of the server. For example, it could be
    /// something like `"192.168.1.1"`.
    /// * `port`: A `String` instance represents the port number on which the server is listening for incoming
    /// connections. Ports are used to uniquely identify different network services running on the same host.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<Client>` containing an instance of a struct.
    async fn ssb_handshake(
        pk: PublicKey,
        sk: SecretKey,
        id: String,
        ip: String,
        port: String,
    ) -> Result<Self> {
        let server_pk = id.replace("=.ed25519", "").replace('@', "");

        let server_pk =
            ed25519::PublicKey::from_slice(&base64::decode(&server_pk)?).expect("bad public key");
        let server_ip_port = format!("{}:{}", ip, port);

        let mut socket = TcpStream::connect(server_ip_port).await?;

        let handshake =
            handshake_client(&mut socket, ssb_net_id(), pk, sk.clone(), server_pk).await?;

        let (box_stream_read, box_stream_write) =
            BoxStream::from_handshake(socket.clone(), socket.clone(), handshake, 0x8000)
                .split_read_write();

        Ok(Self {
            api: ApiCaller::new(RpcWriter::new(box_stream_write)),
            rpc_reader: RpcReader::new(box_stream_read),
            sk,
        })
    }

    /// The method `new` in Rust is an asynchronous method that performs a handshake with a Secure
    /// Scuttlebutt (SSB) server using provided configurations or local secret information.
    ///
    /// # Arguments:
    ///
    /// * `configs`: An optional `OwnedIdentity` struct containing public key (`pk`), secret key (`sk`),
    /// and identity (`id`) information. It can be either `Some` with the provided configuration or
    /// `None`.
    /// * `ip`: A `&str` type representing the IP address
    /// to be used for the handshake process. It is the IP address of the server or device that the
    /// client will be connecting to.
    /// * `port`: A `&str` type representing the port number for the connection.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<Client>` that contains an instance of the `Client` struct or an error.
    pub async fn new(configs: Option<&OwnedIdentity>, ip: &str, port: &str) -> Result<Client> {
        match configs {
            Some(config) => {
                Self::ssb_handshake(
                    config.pk,
                    config.sk.clone(),
                    config.id.clone(),
                    ip.to_string(),
                    port.to_string(),
                )
                .await
            }
            None => {
                let OwnedIdentity { pk, sk, id } =
                    from_patchwork_local().await.expect("read local secret");
                Self::ssb_handshake(pk, sk, id, ip.to_string(), port.to_string()).await
            }
        }
    }

    /// The method `get_secret_key` returns a clone of the secret key associated with the object.
    ///
    /// # Returns:
    ///
    /// Returns an instance of `SecretKey`.
    pub fn get_secret_key(&self) -> SecretKey {
        self.sk.clone()
    }

    /// The `whoami` method in Rust sends a request to the API to retrieve user information and
    /// returns the user's ID.
    ///
    /// # Returns:
    ///
    /// The `whoami` method returns a `Result<String>`, which means it can return either a `String` if
    /// the operation is successful, or an error if there is a problem during the execution.
    pub async fn whoami(&mut self) -> Result<String> {
        let req_id = self.api.whoami_req_send().await?;
        let whoami = self.get_async(req_id, whoami_res_parse).await?.id;

        Ok(whoami)
    }

    /// An asynchronous method that retrieves a message from the SSB network based on the provided message ID.
    ///
    /// # Arguments:
    ///
    /// * `msg_id`: A string slice(`&str`) that represents the ID of the message
    ///
    /// # Returns:
    ///
    /// Returns a `Result<Message>` that contains the messages or an error.
    pub async fn get(&mut self, msg_id: &str) -> Result<Message> {
        let req_id = self.api.get_req_send(msg_id).await?;
        let msg = self.get_async(req_id, message_res_parse).await?;

        Ok(msg)
    }

    /// An asynchronous method that retrieves the list of feed posted by the provided user id.
    ///
    /// # Arguments:
    ///
    /// * `user_id`: A string slice (`&str`) that represents the ID of the user whose feed to be retrieved.
    /// * `live`: A `bool` value that represents whether the feed should be live or not. If `true` the feed will
    /// be continuously fetched from the network.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<Vec<Feed>>` that contains the list of feed posted by the provided user id or an error.
    pub async fn get_feed_by_user(&mut self, live: bool, user_id: &str) -> Result<Vec<Feed>> {
        let user_id = match user_id {
            "me" => self.whoami().await?,
            _ => user_id.to_string(),
        };

        let args = CreateHistoryStreamIn::new(user_id).live(live);

        let req_id = self.api.create_history_stream_req_send(&args).await?;
        let feed = self.get_responses(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    /// An asynchronous method that retrieves the list of all feed posted in the network.
    ///
    /// # Arguments:
    ///
    /// * `live`: A `bool` value that represents whether the feed should be live or not. If `true` the feed will
    /// be continuously fetched from the network.
    ///
    /// # Returns:
    ///
    /// Returns a `Vec<Feed>` that contains the list of all feed posted in the network or an error.
    pub async fn feed(&mut self, live: bool) -> Result<Vec<Feed>> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let feed = self.get_responses(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    /// An asynchronous method that creates a new invite to join the network.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<String>` that contains the invite code or an error.
    pub async fn create_invite(&mut self) -> Result<String> {
        let req_id = self.api.invite_create_req_send(1).await?;
        let responses = self.get_async(req_id, invite_create).await?;
        Ok(responses)
    }

    /// An asynchronous method that accepts an invite to join the network.
    ///
    /// # Arguments:
    ///
    /// * `invite_code`: A string slice(`&str`) type that represents the invite code to be accepted.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<Vec<Feed>>` that contains the list of `contact` feed or an error.
    pub async fn accept_invite(&mut self, invite_code: &str) -> Result<Vec<Feed>> {
        let req_id = self.api.invite_use_req_send(invite_code).await?;
        let responses = self.get_async(req_id, invite_accept_res_parse).await?;
        Ok(responses)
    }

    /// An asynchronous method that publishes a message to the network.
    ///
    /// # Arguments:
    ///
    /// * `msg`: A string slice `&str` type that represents the message to be published.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<()>` that indicates whether the message was published successfully or not.
    pub async fn publish(&mut self, msg: &str, mention: Option<Vec<Mention>>) -> Result<()> {
        let _req_id = self
            .api
            .publish_req_send(TypedMessage::Post {
                text: msg.to_string(),
                mentions: mention,
            })
            .await?;

        Ok(())
    }

    /// An asynchronous method that publishes an `Event` to the network.
    ///
    /// # Arguments:
    ///
    /// * `event`: A string slice (`&str`) type that represents the event to be published.
    /// * `section`: A string slice (`&str`) type that represents the section of the event.
    /// * `content`: A string slice (`&str`) type that represents the content of the event.
    /// * `mentions`: A `Option<Vec<Mention>>` type that represents the mentions of the feed.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<()>` that indicates whether the event was published successfully or not.
    pub async fn publish_event(
        &mut self,
        event: &str,
        section: &str,
        content: &str,
        mentions: Option<Vec<Mention>>,
    ) -> Result<()> {
        let _req_id = self
            .api
            .publish_req_send(TypedMessage::Event {
                event: event.to_string(),
                section: section.to_string(),
                content: content.to_string(),

                mentions,
            })
            .await?;

        Ok(())
    }

    /// An asynchronous method that closes the connection to the network.
    ///
    /// # Returns:
    ///
    /// Returns a `Result<()>` that indicates whether the connection was closed successfully or not.
    pub async fn close(&mut self) -> Result<()> {
        self.api.rpc().close().await?;
        Ok(())
    }
}
