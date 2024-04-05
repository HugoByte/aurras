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
    Ctx, Logger,
};
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use std::io::{Read, Write};

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

    let secret = std::env::var("CONSUMER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| DEFAULT_CONSUMER_PORT.to_string());
    let ip = std::env::var("CONSUMER_IP").unwrap_or(DEFAULT_CONSUMER_IP.to_string());
    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    let ssb_context = context.clone();

    let mut client = Client::new(Some(key.clone()), ip.clone(), port.clone())
        .await
        .unwrap();

    // Spawn the SSB feed listener task
    tokio::spawn(async move {
        client
            .live_feed_with_execution_of_workflow(true, ssb_context.clone())
            .await
            .unwrap();
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
                .await;
            }
            Err(e) => {
                logger.error(&format!("Error accepting connection: {}", e));
            }
        }
    }

    // Keep the main thread alive
    tokio::signal::ctrl_c().await.unwrap();
}

async fn handle_client(
    mut stream: TcpStream,
    ctx: Arc<Mutex<dyn Ctx>>,
    key: OwnedIdentity,
    ip: String,
    port: String,
) {
    let mut client = Client::new(Some(key), ip, port).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Error reading data");

    let body = serde_json::from_slice::<RequestBody>(&buffer).unwrap();

    let ctx = ctx.lock().unwrap();
    let logger = ctx.get_logger().clone();
    logger.info("Data Deserialized");
    let db = ctx.get_db();

    let get_follow_list = db.get_value::<Vec<String>>(FOLLOW_LIST_KEY).unwrap();
    match get_follow_list {
        Some(mut list) => {
            if list.contains(&body.pub_id) {
                logger.info("Already Following the publisher");
            } else {
                list.push(body.pub_id.clone());
                let connect_feed = client.accept_invite(&body.invite).await.unwrap();
                let timestamp = connect_feed[1].value.get("timestamp").unwrap();
                db.insert_value(&format!("{}offset", body.pub_id), timestamp)
                    .unwrap();
                db.insert_value(FOLLOW_LIST_KEY, list).unwrap();
                logger.info(&format!("Following the publisher {}", body.pub_id));
            }
        }
        None => {
            let connect_feed = client.accept_invite(&body.invite).await.unwrap();
            let timestamp = connect_feed[1].value.get("timestamp").unwrap();

            db.insert_value(&format!("{}offset", body.pub_id), timestamp)
                .unwrap();

            db.insert_value(FOLLOW_LIST_KEY, vec![body.pub_id.clone()])
                .unwrap();
            logger.info(&format!("Following the publisher {}", body.pub_id));
        }
    }

    db.insert_value(&body.pub_id.clone(), body).unwrap();
    logger.info("Data inserted successfully");

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
}
