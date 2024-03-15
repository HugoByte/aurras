use super::*;
use serde::{Deserialize, Serialize};

pub struct Client {
    api: ApiCaller<TcpStream>,
    rpc_reader: RpcReader<TcpStream>,
    sk: SecretKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Event {
    id: String,
    body: String,
}

pub struct UserConfig {
    pub public_key: String,
    pub secret_key: String,
    pub id: String,
}

impl UserConfig {
    pub fn new(public_key: String, secret_key: String, id: String) -> Self {
        Self {
            public_key,
            secret_key,
            id,
        }
    }
}

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

    pub async fn user(&mut self, live: bool, user_id: &str) -> Result<()> {
        let user_id = match user_id {
            "me" => self.whoami().await?,
            _ => user_id.to_string(),
        };

        let args = CreateHistoryStreamIn::new(user_id).live(live);

        let req_id = self.api.create_history_stream_req_send(&args).await?;
        self.print_source_until_eof(req_id, feed_res_parse).await?;

        Ok(())
    }

    pub async fn feed(&mut self, live: bool) -> Result<Vec<Feed>> {
        let args = CreateStreamIn::default().live(live);
        let req_id = self.api.create_feed_stream_req_send(&args).await?;

        let feed = self.print_source_until_eof(req_id, feed_res_parse).await?;

        Ok(feed)
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

    pub async fn publish(&mut self, msg: &str) -> Result<()> {
        let _req_id = self
            .api
            .publish_req_send(TypedMessage::Post {
                text: msg.to_string(),
                mentions: None,
            })
            .await?;

        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.api.rpc().close().await?;
        Ok(())
    }

}

// ssb-server should keep running for testing
// use `cargo test -- --ignored` command for testing
#[async_std::test]
#[ignore]
async fn test_client() {
    // passing default ip and port of ssb-server for testing
    let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
        .await
        .unwrap();
    client.user(false, "me").await.unwrap();
}

#[async_std::test]
#[ignore]
async fn test_feed() {
    let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
        .await
        .unwrap();
    client.feed(false).await.unwrap();
}

#[async_std::test]
#[ignore]
async fn test_publish() {
    let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
        .await
        .unwrap();
    let feed = client.feed(false).await.unwrap();
    let prev_len = feed.len();

    let old_event = Event {
        id: "1".to_string(),
        body: "hello_world_event".to_string(),
    };

    let value = serde_json::to_value(old_event.clone()).unwrap();

    let result = client.publish(&value.to_string()).await;
    assert!(result.is_ok());

    // wait for server to publish
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    let feed = client.feed(false).await.unwrap();
    assert!(feed.len() > prev_len);

    let event = feed.last().unwrap().value.clone();
    let message = event.get("content").unwrap();

    let feed_type = message.get("type").unwrap();
    let feed_type: String = serde_json::from_value(feed_type.clone()).unwrap();

    assert_eq!(&feed_type, "post");

    let feed_text = message.get("text").unwrap();
    let feed_text: String = serde_json::from_value(feed_text.clone()).unwrap();

    let new_event: Event = serde_json::from_str(&feed_text).unwrap();
    // let event = serde_json::from_value(event).unwrap();
    assert_eq!(old_event, new_event);
}
