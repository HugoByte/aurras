use async_std::stream::StreamExt;
use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use kuska_ssb::{api::dto::content::Mention, keystore::read_patchwork_config};
use runtime::common::constants::{DEFAULT_PRODUCER_IP, DEFAULT_PRODUCER_PORT};
use runtime::kuska_ssb_client::client::Client;
use runtime::logger::{CoreLogger, Logger};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
pub struct Transfer {
    #[ethevent(indexed)]
    pub from: Address,
    #[ethevent(indexed)]
    pub to: Address,
    pub tokens: U256,
}

#[tokio::main]
async fn main() -> Result<()> {
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
    // let invite_res = client.accept_invite(&invite).await;
    if let Err(e) = client.accept_invite(&invite).await {
        let error = format!("{e:?}");

        if error.contains("already following") {
            logger.info("Already following the pub...");
        } else {
            logger.error(&format!("Error in accepting invite: {:?}", error));
        }
    };

    logger.info("Producer successfully started✅");

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let provider = Arc::new(provider);

    // Subscribe to the Transfer event
    let filter = Filter::new().event("Transfer(address,address,uint256)");
    let event = Transfer::new::<_, Provider<Ws>>(filter, Arc::clone(&provider));
    let mut transfers = event.subscribe().await?;

    while let Some(log) = transfers.next().await {
        logger.info(&format!("Event: {:?}", log));

        // Extract the Transfer data from the Result
        match log {
            Ok(transfer) => {
                let message = format!(
                    "{{\"from\":\"{:?}\",\"to\":\"{:?}\",\"amount\":\"{}\"}}",
                    transfer.from, transfer.to, transfer.tokens
                );

                let mention = Mention {
                    link: pub_address.clone(),
                    name: None,
                };
                let _ = client.publish(&message, Some(vec![mention])).await;
            }
            Err(err) => {
                // Handle the error case (optional)
                println!("Error processing transfer: {}", err);
            }
        }
    }

    Ok(())
}
