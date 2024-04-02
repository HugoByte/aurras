use kuska_ssb::keystore::read_patchwork_config;
use runtime::{
    common::{
        constants::{
            DEFAULT_CONSUMER_IP, DEFAULT_CONSUMER_PORT, FOLLOW_LIST_KEY, HTTP_SERVER_ADDRESS,
        },
        RequestBody,
    },
    constants::DEFAULT_CONSUMER_SECRET_DIR,
    logger::{CoreLogger},
    modules::kuska_ssb_client::client::Client,
    state_manager::GlobalState,
    storage::{CoreStorage, Storage}, Ctx, Logger,
};
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

pub use runtime;
use runtime::Context;

use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = CoreStorage::new("runtime-db").unwrap();
    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let state_manager = GlobalState::new(logger.clone());

    logger.info("starting consumer...");

    let context = Arc::new(Mutex::new(Context::new(logger.clone(), db, state_manager)));

    let secret = std::env::var("CONSUMER_SECRET")
        .unwrap_or_else(|_| DEFAULT_CONSUMER_SECRET_DIR.to_string());
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| DEFAULT_CONSUMER_PORT.to_string());
    let ip = std::env::var("CONSUMER_IP").unwrap_or(DEFAULT_CONSUMER_IP.to_string());
    let key = match async_std::fs::File::open(&secret).await {
        Ok(mut file) => match read_patchwork_config(&mut file).await {
            Ok(key) => key,
            Err(err) => {
                logger.error(&format!("Error reading Patchwork config: {}", err));
                return;
            }
        },
        Err(err) => {
            logger.error(&format!("Error opening the secret file: {}", err));
            return;
        }
    };

    let ssb_context = context.clone();

    let (channel_invite_tx, channel_invite_rx) = std::sync::mpsc::channel::<String>();
    let mut client = match Client::new(Some(key), ip, port).await {
        Ok(client) => client,
        Err(err) => {
            logger.critical(&format!("Error creating SSB client: {}", err));
            return;
        }
    };

    tokio::spawn(async move {
        if let Err(e) = client
            .live_feed_with_execution_of_workflow(true, ssb_context.clone())
            .await
        {
            let log = CoreLogger::new(Some("./ssb-consumer.log"));
            log.error(&format!("Error in live feed processing {}", e));
            return;
        }

        let logger = ssb_context.lock().unwrap().get_logger().clone();
        logger.info("consumer successfully started✅");

        if let Ok(invite) = channel_invite_rx.recv() {
            let res = client.accept_invite(&invite).await;
            logger.info(&format!("Invite Accepted: {:?}", res));
        } else {
            logger.error("Error receiving channel invite");
        }
    });

    // HTTP server with error handling for listener creation
    tokio::spawn(async move {
        let listener = TcpListener::bind(HTTP_SERVER_ADDRESS).expect("Failed to bind to address");
        logger.info(&format!("Listening on {HTTP_SERVER_ADDRESS}..."));
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, context.clone(), channel_invite_tx.clone());
                }
                Err(e) => {
                    logger.error(&format!("Error accepting connection: {}", e));
                }
            }
        }
    });

    // Keep the main thread alive
    tokio::signal::ctrl_c().await.unwrap();
}

use std::io::{Read, Write};

fn handle_client(
    mut stream: TcpStream,
    ctx: Arc<Mutex<dyn Ctx>>,
    channel_invite_tx: std::sync::mpsc::Sender<String>,
) {
    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let mut buffer = Vec::new();
    if let Err(err) = stream.read_to_end(&mut buffer) {
        logger.error(&format!("Error reading data: {}", err));
    };

    let body = match serde_json::from_slice::<RequestBody>(&buffer) {
        Ok(body) => body,
        Err(err) => {
            logger.error(&format!("Error deserialising request body: {}", err));
            return;
        }
    };

    let ctx = match ctx.lock() {
        Ok(ctx) => ctx,
        Err(err) => {
            logger.error(&format!("Error acquiring lock on context: {}", err));
            return;
        }
    };

    let logger = ctx.get_logger().clone();
    logger.info("Data Deserialized");
    let db = ctx.get_db();

    let get_follow_list = match db.get_value::<Vec<String>>(FOLLOW_LIST_KEY) {
        Ok(value) => value,
        Err(err) => {
            logger.error(&format!(
                "Error retrieving follow list from database: {}",
                err
            ));
            return;
        }
    };

    match get_follow_list {
        Some(mut list) => {
            if list.contains(&body.pub_id) {
                logger.info("Already Following the publisher");
            } else {
                list.push(body.pub_id.clone());
                if let Err(err) = channel_invite_tx.send(body.invite.clone()) {
                    logger.error(&format!("Error sending invite: {}", err));
                }
                if let Err(err) = db.insert_value(FOLLOW_LIST_KEY, list) {
                    logger.error(&format!(
                        "Error inserting follow list into database: {}",
                        err
                    ));
                }
                logger.info(&format!("Following the publisher {}", body.pub_id));
            }
        }
        None => {
            if let Err(err) = channel_invite_tx.send(body.invite.clone()) {
                logger.info(&format!("Error sending the invite {}", err));
                eprintln!("Error sending invite: {}", err);
            }
            if let Err(err) = db.insert_value(FOLLOW_LIST_KEY, vec![body.pub_id.clone()]) {
                logger.info(&format!(
                    "Error inserting follow list into database: {}",
                    err
                ));
            }

            logger.info(&format!("Following the publisher {}", body.pub_id));
        }
    }

    if let Err(err) = db.insert_value(&body.pub_id.clone(), body) {
        logger.info(&format!("Error inserting data into database: {}", err));
    }
    logger.info("Data inserted successfully");

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
}
