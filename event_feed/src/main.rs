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
    dotenv::dotenv().ok();
    let config = envy::from_env::<ChainConfig>().unwrap();
    let polkadot = PolkadotFeed::<PolkadotConfig>::new(config).await;
    polkadot
        .event_feed(|e| {
            for i in e {
                println!("{}", i);
            }
        })
        .await;
}
