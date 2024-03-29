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
    let db = CoreStorage::new("runtime_db").unwrap();
    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let state_manager = GlobalState::new(logger.clone());

    logger.info("starting consumer...");

    let context = Arc::new(Mutex::new(Context::new(logger.clone(), db, state_manager)));

    let secret = std::env::var("CONSUMER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| 8008.to_string());
    let key = match async_std::fs::File::open(&secret).await {
        Ok(mut file) => match read_patchwork_config(&mut file).await {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Error reading Patchwork config: {}", err);
                return;
            }
        },
        Err(err) => {
            eprintln!("Error opening secret file: {}", err);
            return;
        }
    };

    let ssb_context = context.clone();

    let (channel_invite_tx, channel_invite_rx) = std::sync::mpsc::channel::<String>();
    let mut client = match Client::new(Some(key), "0.0.0.0".to_string(), port).await {
        Ok(client) => client,
        Err(err) => {
            logger.critical(&format!("Error creating SSB client: {}", err));
            return;
        }
    };

    // Spawn the SSB feed listener task
    tokio::spawn(async move {
        let logger = ssb_context.clone().lock().unwrap().get_logger().clone();
        logger.info("consumer successfully started✅");
        if let Ok(invite) = channel_invite_rx.recv() {
            if let Err(err) = client.accept_invite(&invite).await {
                logger.error(&format!("Error accepting invite: {:?}", err));
                return;
            }
            logger.info("Invite Accepted");
        }

        if let Err(err) = client
            .live_feed_with_execution_of_workflow(true, ssb_context)
            .await
        {
            logger.critical(&format!("Error starting live feed: {:?}", err));
        }
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
    if let Err(err) = stream.read_to_end(&mut buffer) {
        eprintln!("Error reading data: {}", err);
        return;
    };

    let body = match serde_json::from_slice::<RequestBody>(&buffer) {
        Ok(body) => body,
        Err(err) => {
            eprintln!("Error deserializing request body: {}", err);
            return;
        }
    };

    let ctx = match ctx.lock() {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Error acquiring lock on context: {}", err);
            return;
        }
    };

    let logger = ctx.get_logger().clone();
    logger.info("Data Deserialized");
    let db = ctx.get_db();

    let get_follow_list = match db.get_value::<Vec<String>>(FOLLOW_LIST_KEY) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error retrieving follow list from database: {}", err);
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
                    eprintln!("Error sending invite: {}", err);
                    return;
                }
                if let Err(err) = db.insert_value(FOLLOW_LIST_KEY, list) {
                    eprintln!("Error inserting follow list into database: {}", err);
                    return;
                }
                logger.info(&format!("Following the publisher {}", body.pub_id));
            }
        }
        None => {
            if let Err(err) = channel_invite_tx.send(body.invite.clone()) {
                eprintln!("Error sending invite: {}", err);
                return;
            }
            if let Err(err) = db.insert_value(FOLLOW_LIST_KEY, vec![body.pub_id.clone()]) {
                eprintln!("Error inserting follow list into database: {}", err);
                return;
            }

            logger.info(&format!("Following the publisher {}", body.pub_id));
        }
    }

    // db.insert_value(&body.pub_id.clone(), body).unwrap();
    if let Err(err) = db.insert_value(&body.pub_id.clone(), body) {
        eprintln!("Error inserting data into database: {}", err);
        return;
    }
    logger.info("Data inserted successfully");

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
}
