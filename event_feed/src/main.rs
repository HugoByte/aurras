use common::{ChainConfig, EventFeeder};
use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

pub mod common;
mod cosmos;
mod eth;
mod icon;
mod substrate;

#[tokio::main]
async fn main() {
    let config = ChainConfig {
        node_url: "ws://127.0.0.1:9944".to_string(),
        event_filter: String::new(),
        contract_filter: String::new(),
    };

    let polkadot = PolkadotFeed::<PolkadotConfig>::new(config).await;
    polkadot
        .event_feed(|e| {
            for i in e {
                println!("{}", i);
            }
        })
        .await;
}
