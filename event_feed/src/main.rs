use common::{ChainConfig, EventFeeder};
// use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

mod substrate;
mod icon;
mod eth ;
mod cosmos;
pub mod common;

#[tokio::main]
async fn main() {
  
     let config = ChainConfig{ node_url: "ws://127.0.0.1:9944".to_string(), filter: None };
     let polkadot = PolkadotFeed::<PolkadotConfig>::new(config).await;
     polkadot.event_feed(|e| {
      for i in e{
        println!("{}", i);
      }
     }).await;
}