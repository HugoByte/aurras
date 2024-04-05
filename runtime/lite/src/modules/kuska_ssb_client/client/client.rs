use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    modules::{
        logger::Logger, state_manager::traits::GlobalStateManager, storage::Storage,
        wasmtime_wasi_module,
    },
    AppError, Ctx, RequestBody,
};

use super::*;
use kuska_ssb::api::dto::content::{Mention, Post};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct EventId {
    event: Value,
    sender_address: String,
}

impl Hash for EventId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender_address.hash(state);
        match serde_json::to_string(&self.event) {
            Ok(event_string) => event_string.hash(state),
            Err(err) => {
                // Handle serialization error
                eprintln!("Error serializing event data: {}", err);
            }
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

    pub fn get_secret_key(&self) -> SecretKey {
        self.sk.clone()
    }

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
        let mut workflow_map: HashMap<String, RequestBody> = HashMap::new();

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
            let logger = ctx.get_logger();
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
                                        Ok(x) => {
                                            match serde_json::from_str::<serde_json::Value>(&x.text)
                                            {
                                                Ok(mut event) => {
                                                    logger.info(&format!("Event: {:#?}", event));
                                                    let pub_id = &x.mentions.unwrap()[0].link;

                                                    // Check if workflow data exists for this pub_id
                                                    if !workflow_map.contains_key(pub_id) {
                                                        match db.get_request_body(pub_id) {
                                                            Ok(body) => {
                                                                workflow_map.insert(
                                                                    pub_id.to_string(),
                                                                    body,
                                                                );
                                                            }
                                                            Err(e) => logger.trace(&e.to_string()),
                                                        }
                                                    }

                                                    // Get workflow data for this pub_id
                                                    let workflow_data =
                                                        workflow_map.get(pub_id).unwrap();

                                                    let data = serde_json::json!({
                                                        "data": crate::common::combine_values(&mut event, &workflow_data.input),
                                                        "allowed_hosts": workflow_data.allowed_hosts
                                                    });

                                                    let data_value = serde_json::to_value(
                                                        data.clone(),
                                                    )
                                                    .map_err(|err| {
                                                        Box::new(AppError::new(format!(
                                                            "Error converting data to JSON: {}",
                                                            err
                                                        )))
                                                    })?;

                                                    let workflow_index =
                                                        state_manager.new_workflow(0, "hello");

                                                    let _ = wasmtime_wasi_module::run_workflow(
                                                        state_manager,
                                                        logger,
                                                        data_value,
                                                        workflow_data.wasm.clone(),
                                                        workflow_index,
                                                        false,
                                                    );
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
                                    return Err(err.into());
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
