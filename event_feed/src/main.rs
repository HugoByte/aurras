use common::{ChainConfig, EventFeeder};
use substrate::types::PolkadotFeed;
use subxt::PolkadotConfig;

use crate::common::ProducerConfig;

mod substrate;
mod icon;
mod eth ;
mod cosmos;
pub mod common;



#[tokio::main]
async fn main() {
     dotenv::dotenv().ok();
     println!("{}", substrate::start());
     match envy::from_env::<ChainConfig>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
    }

    match envy::from_env::<ProducerConfig>() {
     Ok(config) => println!("{:#?}", config),
     Err(error) => panic!("{:#?}", error)
  }
    
}