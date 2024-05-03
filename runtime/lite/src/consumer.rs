use kuska_ssb::keystore::{read_patchwork_config, OwnedIdentity};
use runtime::{
    common::{
        constants::{
            DEFAULT_CACHE_LIFETIME, DEFAULT_SSB_IP, DEFAULT_SSB_PORT, FOLLOW_LIST_KEY,
            HTTP_SERVER_ADDRESS, UNFOLLOW_LIST_KEY,
        },
        RequestBody,
    },
    logger::CoreLogger,
    modules::kuska_ssb_client::client::Client,
    state_manager::GlobalState,
    storage::{CoreStorage, Storage},
    AppError, Context, Ctx, Logger, RuntimeError,
};

use runtime::common;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    result::Result::{Err, Ok},
    sync::{Arc, Mutex, MutexGuard},
};

/// The main function of the aurras ssb consumer that receives the event feeds from the ssb network
/// and processes them.
#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    dotenv::dotenv().ok();
    let db = CoreStorage::new("runtime-db")?;

    let cache_lifetime = match std::env::var("CACHE_LIFETIME") {
        Ok(value) => value.parse::<u64>().unwrap(),
        Err(_) => DEFAULT_CACHE_LIFETIME,
    };

    let cache_db = CoreStorage::new_with_ttl("./.cache", cache_lifetime, true).unwrap();

    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let state_manager = GlobalState::new(logger.clone());

    logger.info("starting consumer...");

    let context = Arc::new(Mutex::new(Context::new(
        logger.clone(),
        db,
        cache_db,
        state_manager,
    )));

    let secret = std::env::var("CONSUMER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| DEFAULT_SSB_PORT.to_string());
    let ip = std::env::var("CONSUMER_IP").unwrap_or(DEFAULT_SSB_IP.to_string());

    let mut file = async_std::fs::File::open(secret).await?;
    let key = read_patchwork_config(&mut file).await.map_err(|err| {
        RuntimeError::AppError(AppError::new(format!(
            "Error reading Runtime config: {}",
            err
        )))
    })?;

    let ssb_context = context.clone();

    let mut client = match Client::new(Some(&key), &ip, &port).await {
        Ok(client) => client,
        Err(err) => {
            logger.critical(&format!("Error creating SSB client: {}", err));
            return Err(RuntimeError::AppError(AppError::new(format!(
                "Error creating SSB client: {}",
                err
            ))));
        }
    };

    // Spawn the SSB feed listener task
    tokio::spawn(async move {
        client
            .live_feed_with_execution_of_workflow(true, ssb_context.clone())
            .await
            .map_err(|err| {
                Box::new(RuntimeError::AppError(AppError::new(format!(
                    "Error in live feed with execution of workflow: {}",
                    err
                ))))
            })
    });

    // Spawn the HTTP server task
    let listener = TcpListener::bind(HTTP_SERVER_ADDRESS).expect("Failed to bind to address");
    logger.info(&format!("Listening on {HTTP_SERVER_ADDRESS}..."));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(
                    stream,
                    context.clone(),
                    key.clone(),
                    ip.clone(),
                    port.clone(),
                )
                .await?;
            }
            Err(e) => {
                logger.error(&format!("Error accepting connection: {}", e));
            }
        }
    }

    // Keep the main thread alive
    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

/// Handles the process of accepting to the event feed `pub` and storing the workflow wasm
///
/// # Arguments
///
/// * `stream`: A mutable reference to the `TcpStream` to receive the invites.
/// * `ctx`: A runtime context.
/// * `key`: An instance of `OwnedIdentity` that represents the identity of the consumer ssb server.
/// * `ip`: A ssb server ip address of the consumer.
/// * `port`: A ssb server port number of the consumer.
async fn handle_client(
    mut stream: TcpStream,
    ctx: Arc<Mutex<dyn Ctx>>,
    key: OwnedIdentity,
    ip: String,
    port: String,
) -> Result<(), RuntimeError> {
    let mut client = Client::new(Some(&key), &ip, &port).await.map_err(|err| {
        RuntimeError::AppError(AppError::new(format!("Error creating SSB client: {}", err)))
    })?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Error reading data");

    let ctx_guard = ctx.lock()?;
    let logger = ctx_guard.get_logger().clone();

    let body = serde_json::from_slice::<RequestBody>(&buffer)?;
    logger.info("Data Deserialized");

    // Check for unfollow action immediately after deserializing the request body
    if let Some(action) = &body.action {
        logger.debug(&format!("Action received: {}", action));
        if action == "unfollow" {
            let unfollow_result = unfollow_publisher(&ctx_guard, body.pub_id.clone())?;
            if unfollow_result {
                logger.info("Unfollow successful");
                return Ok(());
            } else {
                logger.info("Unfollow not successful, no publisher was unfollowed.");
                return Ok(());
            }
        }
    }

    // Additional checks for follow actions or other operations
    let db = ctx_guard.get_db();
    let unfollowed_list = db
        .get_value::<Vec<String>>(UNFOLLOW_LIST_KEY)?
        .unwrap_or_default();
    if unfollowed_list.contains(&body.pub_id) {
        logger.info(&format!(
            "Publisher {} is unfollowed. Ignoring invite.",
            body.pub_id
        ));
        return Ok(());
    }

    let mut follow_list = db
        .get_value::<Vec<String>>(FOLLOW_LIST_KEY)?
        .unwrap_or_default();
    if follow_list.contains(&body.pub_id) {
        logger.info("Already following the publisher.");
    } else {
        let connect_feed = client.accept_invite(&body.invite).await.map_err(|err| {
            RuntimeError::AppError(AppError::new(format!("Error accepting invite: {}", err)))
        })?;
        let timestamp = connect_feed[1].value.get("timestamp").ok_or_else(|| {
            RuntimeError::AppError(AppError::new(
                "Error getting timestamp from the accepted invite-contact feed".to_string(),
            ))
        })?;
        follow_list.push(body.pub_id.clone());
        db.insert_value(&format!("{}offset", body.pub_id), timestamp)?;
        db.insert_value(FOLLOW_LIST_KEY, follow_list)?;
        logger.info(&format!("Following the publisher {}", body.pub_id));
    }

    // Handle existing data updates
    let existing_data: Option<Vec<RequestBody>> = db.get_value(&body.pub_id.to_string())?;
    match existing_data {
        Some(mut existing_body) => {
            if let Some(position) = existing_body
                .iter()
                .position(|r| r.workflow_id == body.workflow_id)
            {
                existing_body.remove(position);
                existing_body.push(body.clone());
                db.insert_value(&body.pub_id.clone(), existing_body)?;
                logger.info("Existing data updated successfully");
            } else {
                existing_body.push(body.clone());
                db.insert_value(&body.pub_id.clone(), existing_body)?;
                logger.info("Data updated successfully");
            }
        }
        None => {
            let new_data = vec![body.clone()];
            db.insert_value(&body.pub_id.clone(), new_data)?;
            logger.info("Data added");
        }
    }

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
    Ok(())
}

fn unfollow_publisher(ctx: &MutexGuard<dyn Ctx>, pub_id: String) -> Result<bool, RuntimeError> {
    let db = ctx.get_db();
    let logger = ctx.get_logger().clone();

    let follow_list = db
        .get_value::<Vec<String>>(FOLLOW_LIST_KEY)
        .map_err(|e| {
            RuntimeError::AppError(AppError::new(format!("Failed to get follow list: {}", e)))
        })?
        .unwrap_or_default();

    if follow_list.contains(&pub_id) {
        let updated_follow_list = follow_list
            .into_iter()
            .filter(|id| id != &pub_id)
            .collect::<Vec<String>>();

        let mut unfollow_list = db
            .get_value::<Vec<String>>(UNFOLLOW_LIST_KEY)
            .map_err(|e| {
                RuntimeError::AppError(AppError::new(format!("Failed to get unfollow list: {}", e)))
            })?
            .unwrap_or_default();

        unfollow_list.push(pub_id.clone());

        db.insert_value(FOLLOW_LIST_KEY, updated_follow_list)
            .map_err(|e| {
                RuntimeError::AppError(AppError::new(format!(
                    "Failed to update follow list: {}",
                    e
                )))
            })?;
        db.insert_value(UNFOLLOW_LIST_KEY, unfollow_list)
            .map_err(|e| {
                RuntimeError::AppError(AppError::new(format!(
                    "Failed to update unfollow list: {}",
                    e
                )))
            })?;

        logger.info(&format!("Unfollowed publisher {}", pub_id));
        Ok(true)
    } else {
        logger.info(&format!(
            "Cannot unfollow publisher {} as they are not in the follow list.",
            pub_id
        ));
        Ok(false)
    }
}
