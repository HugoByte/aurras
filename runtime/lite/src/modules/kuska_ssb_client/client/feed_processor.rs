use super::*;

use crate::{
    constants::UNFOLLOW_LIST_KEY,
    modules::{
        logger::Logger, state_manager::traits::GlobalStateManager, storage::Storage,
        wasmtime_wasi_module,
    },
    Ctx, RequestBody,
};
use kuska_ssb::api::dto::content::Post;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex, MutexGuard};

impl Client {
    /// Processes the feed and executes workflows based on the events received.
    ///
    /// # Arguments
    ///
    /// * `display`: The feed being processed.
    /// * `logger`: A logger implementation of the `Logger` trait. This is used to log the events received.
    /// * `state_manager`: A state manager implementation of the `GlobalStateManager` trait. This is used to store
    /// and retrieve the state data.
    /// * `db`: A database implementation of the `Storage` trait. Workflows will be retrieved from this database.
    /// * `cache`: A cache implementation of the `Storage` trait. This is used to store the state data.
    fn process_feed<T, U, V>(
        display: Feed,
        logger: &mut MutexGuard<T>,
        state_manager: &mut MutexGuard<U>,
        db: &MutexGuard<V>,
        cache: &MutexGuard<V>,
    ) -> Result<()>
    where
        T: Logger + Clone + std::marker::Send + 'static,
        U: GlobalStateManager,
        V: Storage,
    {
        match serde_json::from_value::<Post>(display.value.get("content").unwrap().clone()) {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content.text) {
                    Ok(event) => {
                        let mentions = &content.mentions.unwrap_or(vec![]);

                        let pub_id = if !mentions.is_empty() {
                            &mentions[0].link
                        } else {
                            return Err(Box::new(AppError::new(format!(
                                "No mentions found in the event: {}",
                                event
                            ))));
                        };

                        // Check unfollow list before processing
                        let unfollowed_list_result = db.get_value::<Vec<String>>(UNFOLLOW_LIST_KEY);
                        let unfollowed_list = match unfollowed_list_result {
                            Ok(Some(list)) => list,
                            Ok(None) => vec![],
                            Err(err) => {
                                logger.error(&format!("Error getting unfollow list: {}", err));
                                return Err(Box::new(AppError::new(
                                    "Error retrieving unfollow list".to_string(),
                                )));
                            }
                        };

                        if unfollowed_list.contains(&pub_id) {
                            logger.info(&format!(
                                "Publisher {} is unfollowed. So workflow cannot be executed.",
                                pub_id
                            ));
                            return Ok(());
                        }

                        let offset = db
                            .get_value::<f64>(&format!("{}offset", pub_id))
                            .map_err(|err| {
                                Box::new(AppError::new(format!(
                                    "offset timestamp not present in the database: {}",
                                    err
                                )))
                            })?
                            .unwrap_or(0.0);

                        let feed_rts = match display.rts {
                            Some(rts) => rts,
                            None => {
                                return Err(Box::new(AppError::new(
                                    "feed received timestamp is none".to_string(),
                                )))
                            }
                        };

                        if offset < feed_rts {
                            logger.info(&format!("Event: {:#?}", event));

                            match db.get_value::<Vec<RequestBody>>(pub_id) {
                                Ok(Some(mut bodies)) => {
                                    // Iterate through bodies, searching for matching pub_id
                                    for body in bodies.iter_mut() {
                                        crate::common::combine_values(&mut body.input, &event);
                                        let data = serde_json::json!({
                                            "data": body.input,
                                            "allowed_hosts": body.allowed_hosts.as_ref(),
                                        });

                                        let workflow_index = state_manager
                                            .new_workflow(&body.workflow_id, &body.workflow_name);
                                        let _ = wasmtime_wasi_module::run_workflow(
                                            state_manager.borrow_mut(),
                                            logger.borrow_mut(),
                                            cache,
                                            data,
                                            body.wasm.clone(),
                                            workflow_index,
                                            false,
                                        );
                                    }
                                }
                                Ok(None) => logger.error("No request bodies found"),
                                Err(e) => logger.error(&format!("Error type: {}", e)),
                            }

                            // Perform database insertion
                            db.insert_value(&format!("{}offset", pub_id), display.timestamp)
                                .map_err(|err| {
                                    Box::new(AppError::new(format!(
                                        "not able to insert offset timestamp to the database: {}",
                                        err
                                    )))
                                })?;
                        }
                    }
                    Err(e) => logger.trace(&e.to_string()),
                }
            }
            Err(e) => logger.trace(&e.to_string()),
        }

        Ok(())
    }

    /// Processes the live feed and executes the workflow based on the events received.
    ///
    /// # Arguments
    ///
    /// * `live` - A boolean indicating whether the feed is live or not.
    /// * `ctx` - Context of the runtime.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the operation was successful or not.
    pub async fn live_feed_with_execution_of_workflow(
        &mut self,
        live: bool,
        ctx: Arc<Mutex<dyn Ctx>>,
    ) -> Result<()> {
        let args = CreateStreamIn::default().live(live);
        let req_no = self.api.create_feed_stream_req_send(&args).await?;
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
            let cache_db = ctx.get_cache_db();
            let mut logger = ctx.get_logger();
            let mut state_manager = ctx.get_state_manager();

            if id == req_no {
                match msg {
                    RecvMsg::RpcResponse(_type, body) => match feed_res_parse(&body) {
                        Ok(display) => {
                            if is_synced {
                                Self::process_feed(
                                    display,
                                    &mut logger,
                                    &mut state_manager,
                                    &db,
                                    &cache_db,
                                )?;
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
                    },
                    RecvMsg::ErrorResponse(message) => return Err(AppError::new(message).into()),
                    _ => {}
                }
            }
        }
    }
}
