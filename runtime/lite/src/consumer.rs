use kuska_ssb::keystore::read_patchwork_config;
use runtime::{
    common::{constants::FOLLOW_LIST_KEY, RequestBody},
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
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| 8008.to_string());
    let ip = std::env::var("CONSUMER_IP").unwrap_or("0.0.0.0".to_string());
    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    let ssb_context = context.clone();

    let (channel_invite_tx, channel_invite_rx) = std::sync::mpsc::channel::<String>();
    let mut client = Client::new(Some(key), ip, port)
        .await
        .unwrap();
    // Spawn the SSB feed listener task
    tokio::spawn(async move {
        let logger = ssb_context.clone().lock().unwrap().get_logger().clone();
        logger.info("consumer successfully started✅");
        {
            let invite = channel_invite_rx.recv().unwrap();
            let res = client.accept_invite(&invite).await;
            logger.info(&format!("Invite Accepted: {:?}", res));
        }

        client
            .live_feed_with_execution_of_workflow(true, ssb_context)
            .await
            .unwrap();
    });

    // Spawn the HTTP server task
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        logger.info("Listening on 127.0.0.1:8080...");
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
                let _res = channel_invite_tx.send(body.invite.clone());
                db.insert_value(FOLLOW_LIST_KEY, list).unwrap();
                logger.info(&format!("Following the publisher {}", body.pub_id));
            }
        }
        None => {
            let _res = channel_invite_tx.send(body.invite.clone());
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
