use kuska_ssb::keystore::{read_patchwork_config, OwnedIdentity};
use runtime::{
    common::{
        constants::{
            DEFAULT_CONSUMER_IP, DEFAULT_CONSUMER_PORT, FOLLOW_LIST_KEY, HTTP_SERVER_ADDRESS,
        },
        RequestBody,
    },
    logger::CoreLogger,
    modules::kuska_ssb_client::client::Client,
    state_manager::GlobalState,
    storage::{CoreStorage, Storage},
    AppError, Ctx, Logger, RuntimeError,
};
use std::result::Result::{Err, Ok};

use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use std::io::{Read, Write};

pub use runtime;
use runtime::Context;

use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    dotenv().ok();
    let db = CoreStorage::new("runtime-db")?;
    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let state_manager = GlobalState::new(logger.clone());

    logger.info("starting consumer...");

    let context = Arc::new(Mutex::new(Context::new(logger.clone(), db, state_manager)));

    let secret = std::env::var("CONSUMER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| DEFAULT_CONSUMER_PORT.to_string());
    let ip = std::env::var("CONSUMER_IP").unwrap_or(DEFAULT_CONSUMER_IP.to_string());

    let mut file = async_std::fs::File::open(secret).await?;
    let key = read_patchwork_config(&mut file).await.map_err(|err| {
        RuntimeError::AppError(AppError::new(format!(
            "Error reading Runtime config: {}",
            err
        )))
    })?;

    let ssb_context = context.clone();

    let mut client = match Client::new(Some(key.clone()), ip.clone(), port.clone()).await {
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

async fn handle_client(
    mut stream: TcpStream,
    ctx: Arc<Mutex<dyn Ctx>>,
    key: OwnedIdentity,
    ip: String,
    port: String,
) -> Result<(), RuntimeError> {
    let mut client = Client::new(Some(key), ip, port).await.map_err(|err| {
        RuntimeError::AppError(AppError::new(format!("Error creating SSB client: {}", err)))
    })?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Error reading data");

    let ctx = ctx.lock()?;
    let logger = ctx.get_logger().clone();

    if let Err(err) = stream.read_to_end(&mut buffer) {
        logger.error(&format!("Error reading data: {}", err));
    };

    let body = serde_json::from_slice::<RequestBody>(&buffer)?;
    logger.info("Data Deserialized");
    let db = ctx.get_db();

    let get_follow_list = db.get_value::<Vec<String>>(FOLLOW_LIST_KEY)?;

    match get_follow_list {
        Some(mut list) => {
            if list.contains(&body.pub_id) {
                // let ctx = ctx.lock()?;
                let logger = ctx.get_logger().clone();
                logger.info("Already Following the publisher");
                return Ok(());
            } else {
                list.push(body.pub_id.clone());
                db.insert_value(FOLLOW_LIST_KEY, list)?;
            }
        }
        None => {
            db.insert_value(FOLLOW_LIST_KEY, vec![body.pub_id.clone()])?;
        }
    }

    let connect_feed = client.accept_invite(&body.invite).await.map_err(|err| {
        RuntimeError::AppError(AppError::new(format!("Error accepting invite: {}", err)))
    })?;
    let timestamp = match connect_feed[1].value.get("timestamp") {
        Some(timestamp) => timestamp,
        None => {
            return Err(RuntimeError::AppError(AppError::new(format!(
                "Error getting timestamp from the accepted invite-contact feed",
            ))));
        }
    };

    db.insert_value(&format!("{}offset", body.pub_id), timestamp)?;
    logger.info(&format!("Following the publisher {}", body.pub_id));

    let existing_data = db.get_request_body(&body.pub_id.to_string())?;

    match existing_data {
        Some(mut existing_body) => {
            if let Some(position) = existing_body.iter().position(|r| r == &body) {
                existing_body.remove(position);
            } else {
                existing_body.push(body.clone());
                db.insert_value(&body.pub_id.clone(), existing_body)?;
                logger.info("Existing data updated successfully");
            }
        }
        None => {
            let new_data = vec![body.clone()];
            db.insert_value(&body.pub_id.clone(), new_data)?;
            logger.info("Data added")
        }
    }

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
    Ok(())
}
