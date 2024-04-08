use std::str::FromStr;

use async_std::stream::StreamExt;
use dotenv::dotenv;
use kuska_ssb::{api::dto::content::Mention, keystore::read_patchwork_config};
use runtime::common::constants::{DEFAULT_PRODUCER_IP, DEFAULT_PRODUCER_PORT};
use runtime::kuska_ssb_client::client::Client;
use runtime::logger::{CoreLogger, Logger};
use web3::futures::StreamExt as _;
use web3::transports::{Http, WebSocket};
use web3::types::{FilterBuilder, Log, H160, H256};
use web3::Web3;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let logger = CoreLogger::new(Some("./ssb-producer.log"));

    logger.info("starting producer...");

    // Load producer secret
    let secret = std::env::var("PRODUCER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });
    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    // Configure producer settings
    let port = std::env::var("PRODUCER_PORT").unwrap_or(DEFAULT_PRODUCER_PORT.to_string());
    let ip = std::env::var("PRODUCER_IP").unwrap_or(DEFAULT_PRODUCER_IP.to_string());
    let pub_address = std::env::var("PUB_ADDRESS").expect("Pub address must be provided");
    let invite = std::env::var("PRODUCER_INVITE").expect("Invite address must be provided");

    // Initialize SSB Client
    let mut client = Client::new(Some(key), ip, port).await.unwrap();

    // Accept SSB invite
    let invite_res = client.accept_invite(&invite).await;
    logger.info(&format!("{:?}", invite_res));
    logger.info("Producer successfully started✅");

    // Initialize Ethereum connection
    let transport =
        WebSocket::new("wss://sepolia.infura.io/ws/v3/97ce164640fd49fc84626007a30c5589")
            .await
            .expect("Ethereum node connection failed");
    let web3 = Web3::new(transport);

    // Insert the 20-byte "from" address in hex format (prefix with 0x)
    let from = H160::from_str("0x9A8c541540950FfdecbBD93B8999261ED4C99a6F").unwrap();

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = H160::from_str("0xa7BC0FAbBc38A7CEdB68aA70C79f8Bbc5057B6Bb").unwrap();

    // Build the tx object
    let tx_object = web3::types::TransactionRequest {
        from,
        to: Some(to),
        value: Some(web3::types::U256::exp10(17)), //0.1 eth
        ..Default::default()
    };

    // Send the tx to sepolia testnet (replace with your desired network)
    let result = web3
        .eth()
        .send_transaction(tx_object)
        .await
        .expect("error in sending");

    println!("Tx succeeded with hash: {}", result);

    // Ethereum filter to monitor transactions
    let filter = FilterBuilder::default()
        .from_block(0.into())
        .address(vec!["0xa86B7F9f891E23fbE7938c8d96b9dB178D5512d9"
            .parse()
            .unwrap()])
        .build();

    // Subscribe to Ethereum new block headers
    let mut subscription = web3
        .eth_subscribe()
        .subscribe_new_heads()
        .await
        .expect("Ethereum subscription failed");

    logger.info(&format!(
        "Subscribed to Ethereum logs with filter: {:?}",
        &filter
    ));

    while let Some(log) = async_std::stream::StreamExt::next(&mut subscription).await {
        match log {
            Ok(log) => {
                // Extract relevant information from the log entry
                let transaction_hash = log.author;
                let block_number = log.hash.unwrap();

                // Prepare SSB message content with Ethereum log information
                let content = Mention {
                    link: pub_address.clone(),
                    name: None,
                };
                let value = format!(
                    "{{\"transaction_hash\":\"{}\", \"block_number\":{}}}",
                    transaction_hash, block_number
                );

                // Publish the message to SSB
                let publish_event = client.publish(&value, Some(vec![content])).await;
                match publish_event {
                    Ok(_) => logger.info("Successfully published Ethereum log event to SSB"),
                    Err(err) => eprintln!("Error publishing Ethereum log event to SSB: {:?}", err),
                }
            }
            Err(err) => eprintln!("Error processing Ethereum log subscription: {:?}", err),
        }
        // Deliberate delay to illustrate asynchronous processing
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
