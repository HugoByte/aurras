use super::*;
use crate::{
    modules::{
        logger::Logger, state_manager::traits::GlobalStateManager, storage::Storage,
        wasmtime_wasi_module,
    },
    AppError, Ctx,
};
use kuska_ssb::api::dto::content::{Mention, Post};
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

/// This Rust code defines an implementation for a `Client` struct with various asynchronous
/// methods for interacting with a remote server using RPC (Remote Procedure Call) communication. Here
/// is a summary of what each method does:
impl Client {
    /// The `get_async` function in Rust is an asynchronous method that receives messages and processes
    /// them based on the request number and message type.
    /// 
    /// Arguments:
    /// 
    /// * `req_no`: The `req_no` parameter in the `get_async` function represents the request number
    /// that the function is waiting for. The function will keep looping and checking incoming messages
    /// until it receives a response with the matching request number.
    /// * `f`: The parameter `f` in the `get_async` function is a closure that takes a reference to a
    /// slice of `u8` bytes as input and returns a `Result<T>`, where `T` implements the `Debug` trait.
    /// This closure is used to process the body of the received
    /// 
    /// Returns:
    /// 
    /// The `get_async` function returns a `Result<T>`, where `T` is the type specified in the function
    /// signature. The result can either be an `Ok` variant containing the value of type `T` if the
    /// operation is successful, or an `Err` variant containing an error if an error occurs during the
    /// execution of the function.
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

    /// This Rust function asynchronously receives and processes responses based on a provided closure,
    /// handling different message types and error cases.
    /// 
    /// Arguments:
    /// 
    /// * `req_no`: The `req_no` parameter in the `get_responses` function represents the request number
    /// that is being processed. It is used to match the incoming messages with the corresponding
    /// request for processing.
    /// * `f`: The `f` parameter in the `get_responses` function is a closure that takes a slice of
    /// bytes (`&[u8]`) as input and returns a result of type `T`. The closure is used to deserialize
    /// the byte slice into a value of type `T`. The `where F
    /// 
    /// Returns:
    /// 
    /// The function `get_responses` returns a `Result` containing a vector of type `T`.
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

   /// The function `ssb_handshake` establishes a secure Scuttlebutt handshake with a server using
   /// provided keys and connection details.
   /// 
   /// Arguments:
   /// 
   /// * `pk`: PublicKey of the client
   /// * `sk`: The `sk` parameter in the `ssb_handshake` function is the secret key used for the
   /// handshake process. It is typically a cryptographic key that is kept secret and used for secure
   /// communication between the client and server. In this context, it is likely an Ed25519 secret key
   /// used for
   /// * `id`: The `id` parameter in the `ssb_handshake` function represents the identity of the server
   /// in the Secure Scuttlebutt (SSB) protocol. It is used to uniquely identify the server and
   /// establish a secure connection between the client and the server. The `id` parameter is expected
   /// to
   /// * `ip`: The `ip` parameter in the `ssb_handshake` function represents the IP address of the
   /// server you want to connect to. It is a string that contains the IPv4 or IPv6 address of the
   /// server. For example, it could be something like `"192.168.1.1
   /// * `port`: The `port` parameter in the `ssb_handshake` function represents the port number on
   /// which the server is listening for incoming connections. Ports are used to uniquely identify
   /// different network services running on the same host. Common ports include 80 for HTTP, 443 for
   /// HTTPS, and 22 for
   /// 
   /// Returns:
   /// 
   /// The `ssb_handshake` function is returning a `Result` containing an instance of a struct. The
   /// struct being returned has three fields: `api` of type `ApiCaller`, `rpc_reader` of type
   /// `RpcReader`, and `sk` of type `SecretKey`.
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

    /// The function `new` in Rust is an asynchronous function that performs a handshake with a Secure
    /// Scuttlebutt (SSB) server using provided configurations or local secret information.
    /// 
    /// Arguments:
    /// 
    /// * `configs`: An optional `OwnedIdentity` struct containing public key (`pk`), secret key (`sk`),
    /// and identity (`id`) information. It can be either `Some` with the provided configuration or
    /// `None`.
    /// * `ip`: The `ip` parameter in the function `new` is a `String` type representing the IP address
    /// to be used for the handshake process. It is the IP address of the server or device that the
    /// client will be connecting to.
    /// * `port`: The `port` parameter in the `new` function is a `String` type. It is used to specify
    /// the port number for the connection.
    /// 
    /// Returns:
    /// 
    /// The `new` function returns a `Result<Client>`.
    pub async fn new(configs: Option<OwnedIdentity>, ip: String, port: String) -> Result<Client> {
        match configs {
            Some(config) => Self::ssb_handshake(config.pk, config.sk, config.id, ip, port).await,
            None => {
                let OwnedIdentity { pk, sk, id } =
                    from_patchwork_local().await.expect("read local secret");
                Self::ssb_handshake(pk, sk, id, ip, port).await
            }
        }
    }

    /// The function `get_secret_key` returns a clone of the secret key associated with the object.
    /// 
    /// Returns:
    /// 
    /// A `SecretKey` is being returned.
    pub fn get_secret_key(&self) -> SecretKey {
        self.sk.clone()
    }

    /// The `whoami` function in Rust sends a request to the API to retrieve user information and
    /// returns the user's ID.
    /// 
    /// Returns:
    /// 
    /// The `whoami` function returns a `Result<String>`, which means it can return either a `String` if
    /// the operation is successful, or an error if there is a problem during the execution.
    pub async fn whoami(&mut self) -> Result<String> {
        let req_id = self.api.whoami_req_send().await?;
        let whoami = self.get_async(req_id, whoami_res_parse).await?.id;

        Ok(whoami)
    }

    pub async fn get(&mut self, msg_id: &str) -> Result<Message> {
        let req_id = self.api.get_req_send(msg_id).await?;
        let msg = self.get_async(req_id, message_res_parse).await?;

        Ok(msg)
    }

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

    pub async fn feed(&mut self, live: bool) -> Result<Vec<Feed>> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let feed = self.get_responses(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    pub async fn live_feed_with_execution_of_workflow(
        &mut self,
        live: bool,
        ctx: Arc<Mutex<dyn Ctx>>,
    ) -> Result<()> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        self.execute_workflow_by_event(req_id, ctx).await?;

        Ok(())
    }

    pub async fn create_invite(&mut self) -> Result<String> {
        let req_id = self.api.invite_create_req_send(1).await?;
        let responses = self.get_async(req_id, invite_create).await?;
        Ok(responses)
    }

    pub async fn accept_invite(&mut self, invite_code: &str) -> Result<Vec<Feed>> {
        let req_id = self.api.invite_use_req_send(invite_code).await?;
        let responses = self.get_async(req_id, invite_accept_res_parse).await?;
        Ok(responses)
    }

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

    pub async fn close(&mut self) -> Result<()> {
        self.api.rpc().close().await?;
        Ok(())
    }

    pub async fn execute_workflow_by_event(
        &mut self,
        req_no: RequestNo,
        ctx: Arc<Mutex<dyn Ctx>>,
    ) -> Result<()> {
        let mut is_synced = false;

        loop {
            let (id, msg) = self.rpc_reader.recv().await?;
            let ctx = match ctx.lock() {
                Ok(ctx) => ctx,
                Err(err) => {
                    return Err(Box::new(AppError::new(format!(
                        "Failed to acquire lock on context: {}",
                        err
                    ))));
                }
            };
            let db = ctx.get_db();
            let mut logger = ctx.get_logger();
            let mut state_manager = ctx.get_state_manager();

            if id == req_no {
                match msg {
                    RecvMsg::RpcResponse(_type, body) => {
                        let display = feed_res_parse(&body);

                        match display {
                            Ok(display) => {
                                if is_synced {
                                    match serde_json::from_value::<Post>(
                                        display.value.get("content").unwrap().clone(),
                                    ) {
                                        Ok(content) => {
                                            match serde_json::from_str::<serde_json::Value>(
                                                &content.text,
                                            ) {
                                                Ok(event) => {
                                                    let mentions =
                                                        &content.mentions.unwrap_or(vec![]);

                                                    let pub_id = if !mentions.is_empty() {
                                                        &mentions[0].link
                                                    } else {
                                                        return Err(Box::new(AppError::new(
                                                            format!("No mentions found in the event: {}", event)
                                                        )));
                                                    };

                                                    let offset = db
                                                        .get_value::<f64>(&format!(
                                                            "{}offset",
                                                            pub_id
                                                        ))
                                                        .map_err(|err| {
                                                            Box::new(AppError::new(
                                                                format!(
                                                            "offset timestamp not present in the database: {}",
                                                                err
                                                            ),
                                                            ))
                                                        })?
                                                        .unwrap_or(0.0);

                                                    let feed_rts = match display.rts {
                                                        Some(rts) => rts,
                                                        None => {
                                                            return Err(Box::new(AppError::new(
                                                                "feed received timestamp is none"
                                                                    .to_string(),
                                                            )))
                                                        }
                                                    };

                                                    if offset < feed_rts {
                                                        logger
                                                            .info(&format!("Event: {:#?}", event));

                                                        match db.get_request_body(pub_id) {
                                                            Ok(bodies) => {
                                                                // Handle the case where bodies is None (no request bodies found)
                                                                if bodies.is_none() {
                                                                    logger.error(&format!(
                                                                        "No request bodies found"
                                                                    ));
                                                                }

                                                                let mut bodies = bodies.unwrap();

                                                                // Iterate through bodies, searching for matching pub_id
                                                                for body in bodies.iter_mut() {
                                                                    crate::common::combine_values(
                                                                        &mut body.input,
                                                                        &event,
                                                                    );
                                                                    let data = serde_json::json!({
                                                                        "data": body.input,
                                                                        "allowed_hosts": body.allowed_hosts.as_ref(),
                                                                    });

                                                                    let workflow_index =
                                                                        state_manager.new_workflow(
                                                                            &body.workflow_id,
                                                                            &body.workflow_name,
                                                                        );
                                                                    let _ = wasmtime_wasi_module::run_workflow(
                                                                            state_manager.borrow_mut(),
                                                                            logger.borrow_mut(),
                                                                            data,
                                                                            body.wasm.clone(),
                                                                            workflow_index,
                                                                            false,
                                                                        );
                                                                }
                                                            }
                                                            Err(e) => logger.error(&format!(
                                                                "Error type: {}",
                                                                e
                                                            )),
                                                        }

                                                        // Perform database insertion
                                                        db.insert_value(&format!("{}offset", pub_id), display.timestamp)
                                                            .map_err(|err| {
                                                                Box::new(AppError::new(format!(
                                                                    "not able to insert offset timestamp to the database: {}", err
                                                                )))
                                                            })?;
                                                    }
                                                }
                                                Err(e) => logger.trace(&e.to_string()),
                                            }
                                        }
                                        Err(e) => logger.trace(&e.to_string()),
                                    }
                                }
                            }
                            Err(err) => {
                                let body = std::str::from_utf8(&body).unwrap();

                                if body == "{\"sync\":true}" {
                                    logger.info("Syncing Successful");
                                    is_synced = true;
                                } else {
                                    return Err(err);
                                }
                            }
                        }
                    }
                    RecvMsg::ErrorResponse(message) => {
                        return Err(AppError::new(message).into());
                    }
                    _ => {}
                }
            }
        }
    }
}
