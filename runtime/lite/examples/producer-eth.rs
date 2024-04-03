use web3::futures::StreamExt as _;
use web3::transports::{Http, WebSocket};
use web3::types::{FilterBuilder, Log};
use web3::Web3;
// use ssb::{FeedId, Ssb};
use dotenv::dotenv;
use kuska_ssb::{api::dto::content::Mention, keystore::read_patchwork_config};
use runtime::constants::DEFAULT_PRODUCER_SECRET_DIR;
use runtime::kuska_ssb_client::client::Client;
use runtime::logger::{CoreLogger, Logger};
use runtime::common::constants::{DEFAULT_PRODUCER_IP, DEFAULT_PRODUCER_PORT};

#[tokio::main]
async fn main() {

    dotenv().ok();

    let logger = CoreLogger::new(Some("./ssb-producer.log"));

    logger.info("starting producer...");
    let secret = std::env::var("PRODUCER_SECRET").unwrap_or_else(|_| {
        DEFAULT_PRODUCER_SECRET_DIR.to_string()
    });

    let port = std::env::var("PRODUCER_PORT").unwrap_or(DEFAULT_PRODUCER_PORT.to_string());
    let ip = std::env::var("PRODUCER_IP").unwrap_or(DEFAULT_PRODUCER_IP.to_string());
    let pub_address = std::env::var("PUB_ADDRESS").expect("Pub address must be provided");
    let invite = std::env::var("PRODUCER_INVITE").expect("invite address must be provided");

    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    // Initialize Ethereum connection
    let transport = WebSocket::new("wss://mainnet.infura.io/ws/v3/97ce164640fd49fc84626007a30c5589").await.expect("Ethereum node connection failed");
    let web3 = Web3::new(transport);

    // Initialize SSB connection
    // let ssb = Ssb::new().expect("SSB connection failed");
    let mut client = Client::new(Some(key), ip, port)
        .await
        .unwrap();

    let invite_res = client.accept_invite(&invite).await;
    logger.info(&format!("{:?}", invite_res));
    logger.info("producer successfully started✅");

    // Ethereum filter to monitor transactions
    let filter = FilterBuilder::default()
        .from_block(0.into())
        .address(vec!["your_contract_address".parse().unwrap()])
        .build();

    // Web3 subscription to Ethereum logs
    let subscription = web3.eth_subscribe().subscribe_logs(filter).await.expect("Subscribing to logs failed");
    
    
    // Process Ethereum logs
    let mut stream = Box::pin(subscription.filter_map(|log| async move{
        match log {
            Ok(log) => Some(log),
            Err(err) => {
                eprintln!("Error processing log: {:?}", err);
                None
            }
        }

    }));

    while let Some(log) = stream.as_mut().next().await {
        let transaction_hash = log.transaction_hash.unwrap().to_string();
        let sender = log.topics[1].clone().to_string();
        let receiver = log.data.clone();
        let value = log.clone();

        // Prepare SSB message content
        let content = Mention { link: "mention".to_string(), name: format!("Transaction Hash:{}", transaction_hash).into() };

            let content_to_publish = vec![content];

        // Publish the message to SSB
        let publish_result = client.publish(&pub_address, Some(content_to_publish)).await;
        match publish_result {
            Ok(_) => logger.info("Successfully published event to SSB"),
            Err(err) => eprintln!("Error publishing to SSB: {:?}", err),
        }
       
        // Example: Print transaction details
        println!("Transaction Hash: {}", transaction_hash);
        println!("Sender: {}", sender);
        println!("Receiver: {:?}", receiver);
        println!("Value: {:?}", value);

        // Deliberate delay to illustrate asynchronous processing
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}





    
















