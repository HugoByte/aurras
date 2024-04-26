use crate::common::Context;
use crate::common::Producer;
use common::cosmos::CosmosFeed;
use common::{ChainConfig, ProducerConfig};
use icon::IconFeed;
use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

pub mod common;
mod cosmos;
mod eth;
mod icon;
mod substrate;
// pub use cosmos;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let chain_config = envy::from_env::<ChainConfig>().unwrap();
    let producer_config = envy::from_env::<ProducerConfig>().unwrap();

    let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
    ssb_client.accept_invite().await.unwrap();

    let feed: Context = match chain_config.clone().chain.to_lowercase().as_str() {
        "substrate" => {
            let polkadot_client = PolkadotFeed::<PolkadotConfig>::new(chain_config.clone()).await.unwrap();
            Context::PolkadotFeed(polkadot_client)
        }
        "icon" => {
            let icon_client = IconFeed::new(chain_config.clone()).unwrap();
            Context::IconFeed(icon_client)
        }
        _ => panic!("Invalid Chain"),
    };

    let _ = feed.feed_events(&|e| {
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
