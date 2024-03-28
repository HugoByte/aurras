use kuska_ssb::keystore::read_patchwork_config;
use runtime::{
    common::RequestBody,
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
    let db = CoreStorage::new("runtime").unwrap();
    let logger = CoreLogger::new(Some("./ssb-consumer.log"));
    let state_manager = GlobalState::new(logger.clone());

    logger.info("starting consumer...");

    let context = Arc::new(Mutex::new(Context::new(
        logger.clone(),
        db,
        state_manager
    )));

    let secret = std::env::var("CONSUMER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let port = std::env::var("CONSUMER_PORT").unwrap_or_else(|_| 8008.to_string());
    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    let ssb_context = context.clone();

    // Spawn the SSB feed listener task
    tokio::spawn(async move{
        let mut client = Client::new(Some(key), "0.0.0.0".to_string(), port)
            .await
            .unwrap();

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
                    handle_client(stream, context.clone());
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

fn handle_client(mut stream: TcpStream, ctx: Arc<Mutex<dyn Ctx>>) {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Error reading data");

    let body = serde_json::from_slice::<RequestBody>(&buffer).unwrap();

    let ctx = ctx.lock().unwrap();
    let logger = ctx.get_logger().clone();
    logger.info("Data Deserialized");
    let db = ctx.get_db();

    db.insert_request_body(&body.pub_id.clone(), body).unwrap();
    logger.info("Data inserted successfully");

    // Respond to the client (optional)
    let response = "Data received!";
    stream
        .write_all(response.as_bytes())
        .expect("Error writing response");
}
