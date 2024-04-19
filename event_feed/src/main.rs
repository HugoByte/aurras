use crate::common::Producer;
use common::{ChainConfig, EventFeeder, ProducerConfig};
use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

pub mod common;
mod cosmos;
mod eth;
mod icon;
mod substrate;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let chain_config = envy::from_env::<ChainConfig>().unwrap();
    let producer_config = envy::from_env::<ProducerConfig>().unwrap();

    let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
    ssb_client.accept_invite().await.unwrap();

    let polkadot = PolkadotFeed::<PolkadotConfig>::new(chain_config).await;
    polkadot
        .event_feed(&|e| {
            for i in e {
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
