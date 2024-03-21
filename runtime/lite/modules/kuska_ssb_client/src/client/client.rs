use super::*;
use kuska_ssb::api::dto::content::Mention;

impl Client {
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
                        return f(&body).map_err(|err| err.into());
                    }
                    RecvMsg::ErrorResponse(message) => {
                        return std::result::Result::Err(Box::new(AppError::new(message)));
                    }
                    _ => {}
                }
            }
        }
    }

    async fn print_source_until_eof<'a, T, F>(&mut self, req_no: RequestNo, f: F) -> Result<Vec<T>>
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
                            Ok(display) => {
                                println!("response=> {:#?}", display);
                                response.push(display);
                            }
                            Err(err) => {
                                let body = std::str::from_utf8(&body).unwrap();

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
                    RecvMsg::CancelStreamRespose() => break,
                    _ => {}
                }
            }
        }

        Ok(response)
    }

    async fn ssb_handshake(
        pk: PublicKey,
        sk: SecretKey,
        id: String,
        ip: String,
        port: String,
    ) -> Result<Self> {
        let server_pk = id.replace("=.ed25519", "").replace("@", "");

        let server_pk =
            ed25519::PublicKey::from_slice(&base64::decode(&server_pk)?).expect("bad public key");
        let server_ipport = format!("{}:{}", ip, port);

        let mut socket = TcpStream::connect(server_ipport).await?;

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

    pub async fn new(configs: Option<UserConfig>, ip: String, port: String) -> Result<Client> {
        match configs {
            Some(config) => {
                let public_key =
                    PublicKey::from_slice(&base64::decode(&config.public_key)?).unwrap();
                let secret_key =
                    SecretKey::from_slice(&base64::decode(&config.secret_key)?).unwrap();
                let id = config.id;

                Self::ssb_handshake(public_key, secret_key, id, ip, port).await
            }
            None => {
                let OwnedIdentity { pk, sk, id } =
                    from_patchwork_local().await.expect("read local secret");
                Self::ssb_handshake(pk, sk, id, ip, port).await
            }
        }
    }

    pub fn get_secret_key(&self) -> SecretKey {
        self.sk.clone()
    }

    pub async fn whoami(&mut self) -> Result<String> {
        let req_id = self.api.whoami_req_send().await?;
        let whoami = self.get_async(req_id, whoami_res_parse).await?.id;

        Ok(whoami)
    }

    pub async fn get(&mut self, msg_id: &str) -> Result<Message> {
        let msg_id = if msg_id == "any" {
            "%TL34NIX8JpMJN+ubHWx6cRhIwEal8VqHdKVg2t6lFcg=.sha256".to_string()
        } else {
            msg_id.to_string()
        };

        let req_id = self.api.get_req_send(&msg_id).await?;
        let msg = self.get_async(req_id, message_res_parse).await?;

        Ok(msg)
    }

    pub async fn user(&mut self, live: bool, user_id: &str) -> Result<Vec<Feed>> {
        let user_id = match user_id {
            "me" => self.whoami().await?,
            _ => user_id.to_string(),
        };

        let args = CreateHistoryStreamIn::new(user_id).live(live);

        let req_id = self.api.create_history_stream_req_send(&args).await?;
        let feed = self.print_source_until_eof(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    pub async fn feed(&mut self, live: bool) -> Result<Vec<Feed>> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let feed = self.print_source_until_eof(req_id, feed_res_parse).await?;

        Ok(feed)
    }

    pub async fn live_feed_with_execution_of_workflow(&mut self, live: bool) -> Result<()> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let _feed = self.execute_workflow_by_event(req_id).await?;

        Ok(())
    }

    pub async fn latest(&mut self) -> Result<()> {
        let req_id = self.api.latest_req_send().await?;
        self.print_source_until_eof(req_id, latest_res_parse)
            .await?;

        Ok(())
    }

    pub async fn private(&mut self, user_id: &str) -> Result<()> {
        let user_id = match user_id {
            "me" => self.whoami().await?,
            _ => user_id.to_string(),
        };

        let sk = self.get_secret_key();
        let show_private = |body: &[u8]| {
            let msg = feed_res_parse(body)?.into_message()?;
            if let serde_json::Value::String(content) = msg.content() {
                if is_privatebox(&content) {
                    let ret = privatebox_decipher(&content, &sk)?.unwrap_or("".to_string());
                    return Ok(ret);
                }
            }
            return Ok("".to_string());
        };

        let args = CreateHistoryStreamIn::new(user_id.to_string());
        let req_id = self.api.create_history_stream_req_send(&args).await?;

        self.print_source_until_eof(req_id, show_private).await?;

        Ok(())
    }

    pub async fn create_invite(&mut self) -> Result<()> {
        let req_id = self.api.invite_create_req_send(1).await?;

        self.print_source_until_eof(req_id, invite_create).await?;

        Ok(())
    }
    pub async fn accept_invite(&mut self, invite_code: &str) -> Result<()> {
        let req_id = self.api.invite_use_req_send(invite_code).await?;

        self.print_source_until_eof(req_id, invite_create).await?;
        Ok(())
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

    pub async fn close(&mut self) -> Result<()> {
        self.api.rpc().close().await?;
        Ok(())
    }

    async fn execute_workflow_by_event(&mut self, req_no: RequestNo) -> Result<()> {
        let mut response = vec![];
        loop {
            let (id, msg) = self.rpc_reader.recv().await?;

            if id == req_no {
                match msg {
                    RecvMsg::RpcResponse(_type, body) => {
                        let display = feed_res_parse(&body);

                        match display {
                            Ok(display) => {
                                match serde_json::from_value::<Content>(
                                    display.value.get("content").unwrap().clone(),
                                ) {
                                    Ok(x) => match serde_json::from_str::<Event>(&x.text) {
                                        Ok(event) => {
                                            response.push(display);
                                            println!("{:#?}", event);

                                            if event.id == "1".to_string() {
                                                //TODO
                                                // Setup a mechnism to read the workflow and input
                                               let _ = wasmtime_wasi_module::run_workflow(
                                                    serde_json::to_value(event.body).unwrap(),
                                                    vec![],
                                                    0,
                                                    "hello"
                                                );
                                            }
                                        }
                                        Err(e) => println!("{:#?}", e),
                                    },
                                    Err(e) => println!("{:#?}", e),
                                }
                            }
                            Err(err) => {
                                let body = std::str::from_utf8(&body).unwrap();

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
                    RecvMsg::CancelStreamRespose() => {}
                    _ => {}
                }
            }
        }
    }
}
