use std::borrow::BorrowMut;
use std::sync::Arc;

use crate::common::Context;
use crate::common::Producer;
use common::{ChainConfig, EventFeeder, ProducerConfig};
use icon::IconFeed;
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

    match chain_config.clone().chain.to_lowercase().as_str() {
        "substrate" => {
            let s = Arc::new(PolkadotFeed::<PolkadotConfig>::new(chain_config.clone()).await);
            let feed = Context::new(s);
            feed.feed_client
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
        "icon" => {
            let s = Arc::new(IconFeed::new(chain_config.clone()).unwrap());
            let feed = Context::new(s);
            feed.feed_client
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
                .await.unwrap();
        }
        _ => todo!(),
    };

    let feed: Context<dyn EventFeeder> = match chain_config.clone().chain.to_lowercase().as_str() {
        "substrate" => {
            let s = Arc::new(PolkadotFeed::<PolkadotConfig>::new(chain_config.clone()).await);
            Context::new(s)
        }
        "icon" => {
            let s = Arc::new(IconFeed::new(chain_config.clone()).unwrap());
            Context::new(s)
        }
        _ => todo!(),
    };

}
