use crate::common::Context;
use crate::common::Producer;
use common::cosmos::CosmosFeed;
use common::{ChainConfig, ProducerConfig};
use eth::EthFeed;
use icon::IconFeed;
use runtime::{logger::CoreLogger, Logger};
use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

pub mod common;
mod cosmos;
mod eth;
mod icon;
mod substrate;

#[tokio::main]
async fn main() {
    let logger = CoreLogger::new(Some("./event-feed.log"));
    dotenv::dotenv().ok();
    let chain_config = envy::from_env::<ChainConfig>().unwrap();
    let producer_config = envy::from_env::<ProducerConfig>().unwrap();

    let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
    logger.info("SSB client created");
    ssb_client.accept_invite().await.unwrap();

    let feed: Context = match chain_config.clone().chain.to_lowercase().as_str() {
        "substrate" => {
            let polkadot_client = PolkadotFeed::<PolkadotConfig>::new(chain_config.clone())
                .await
                .unwrap();
            logger.info("Polkadot client got created");
            Context::PolkadotFeed(polkadot_client)
        }
        "icon" => {
            let icon_client = IconFeed::new(chain_config.clone()).unwrap();
            logger.info("Icon client got created");
            Context::IconFeed(icon_client)
        }
        "eth" => {
            let eth_client = EthFeed::new(chain_config.clone()).await.unwrap();
            logger.info("Eth client got created");
            Context::EthFeed(eth_client)
        }
        "cosmos" => {
            let cosmos_client = CosmosFeed::new(chain_config.clone());
            logger.info("Cosmos client got created");
            Context::CosmosFeed(cosmos_client)
        }
        _ => panic!("Invalid Chain"),
    };

    let _ = feed
        .feed_events(&|e| {
            for i in e {
                logger.info(&format!("Process the events : {:?}", i));
                tokio::spawn(async move {
                    let producer_config = envy::from_env::<ProducerConfig>().unwrap();
                    let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
                    ssb_client
                        .publish_feed(i)
                        .await
                        .expect("Failed to send event");
                });
            }
        })
        .await;
}
