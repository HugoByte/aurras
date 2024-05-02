use super::*;
use kuska_ssb::api::dto::content::Mention;

impl Client {
    /// Receives a single response for the given request number from the rpc reader and 
    /// parses them based on a provided closure. 
    ///
    /// # Arguments:
    ///
    /// * `req_no`: An request number that uniquely identifies the request.
    /// * `f`: A closure to parse the received message.
    ///
    /// # Returns:
    ///
    /// Returns the result of the parsing closure.
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

    /// Receives multiple responses for the given request number from the rpc reader and 
    /// parses them based on a provided closure.
    ///
    /// # Arguments:
    ///
    /// * `req_no`: A request number that uniquely identifies the request.
    /// * `f`: A closure to parse the received message.
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

    /// Performs the handshake protocol to establish a secure connection with the ssb server.
    ///
    /// # Arguments:
    ///
    /// * `pk`: SSB public key.
    /// * `sk`: SSB secret key.
    /// * `id`: SSB id.
    /// * `ip`: IP address of the ssb server.
    /// * `port`: Port number of the ssb server.
    ///
    /// # Returns:
    ///
    /// Returns the client.
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

    /// Creates a new client for the ssb server.
    ///
    /// # Arguments:
    ///
    /// * `configs`: SSB client configurations such as public key, secret key, and id. Local server
    /// defaults are used if `configs` is `None`.
    /// * `ip`: SSB server IP address.
    /// * `port`: SSB server port number.
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

    /// Returns the secret key associated with the SSB client.
    pub fn get_secret_key(&self) -> SecretKey {
        self.sk.clone()
    }

    /// Returns the public key associated with the SSB client.
    pub async fn whoami(&mut self) -> Result<String> {
        let req_id = self.api.whoami_req_send().await?;
        let whoami = self.get_async(req_id, whoami_res_parse).await?.id;

        Ok(whoami)
    }

    /// Retrieves a message from the SSB network based on the provided message ID.
    ///
    /// # Arguments:
    ///
    /// * `msg_id`: ssb feed message id.
    pub async fn get(&mut self, msg_id: &str) -> Result<Message> {
        let req_id = self.api.get_req_send(msg_id).await?;
        let msg = self.get_async(req_id, message_res_parse).await?;

        Ok(msg)
    }

    /// Retrieves the list of feed posted by the provided user id.
    ///
    /// # Arguments:
    ///
    /// * `user_id`: SSB user id.
    /// * `live`: A `bool` value that represents whether the feed should be live or not.
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

    /// Retrieves a list of feed posted in the network.
    ///
    /// # Arguments:
    ///
    /// * `live`: A `bool` value that represents whether the feed should be live or not.
    pub async fn feed(&mut self, live: bool) -> Result<Vec<Feed>> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let feed = self.get_responses(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    /// Returns a new invite to join the ssb network.
    pub async fn create_invite(&mut self) -> Result<String> {
        let req_id = self.api.invite_create_req_send(1).await?;
        let responses = self.get_async(req_id, invite_create).await?;
        Ok(responses)
    }

    /// Accepts an invite to join other ssb networks.
    ///
    /// # Arguments:
    ///
    /// * `invite_code`: SSB invite key to be accepted.
    ///
    /// # Returns:
    ///
    /// Returns a list of `Feed` that represents the accepted invite.
    pub async fn accept_invite(&mut self, invite_code: &str) -> Result<Vec<Feed>> {
        let req_id = self.api.invite_use_req_send(invite_code).await?;
        let responses = self.get_async(req_id, invite_accept_res_parse).await?;
        Ok(responses)
    }

    /// Publishes a message to the network.
    ///
    /// # Arguments:
    ///
    /// * `msg`: The message to be published.
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

    /// Publishes an `Event` as a message to the network.
    ///
    /// # Arguments:
    ///
    /// * `event`: Event to be published.
    /// * `section`: Section of the event.
    /// * `content`: Content of the event.
    /// * `mentions`: An optional mentions of the feed.
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

    /// Closes the connection inbetween the client and the ssb network.
    pub async fn close(&mut self) -> Result<()> {
        self.api.rpc().close().await?;
        Ok(())
    }
}
