// pub mod types;

use dotenv::dotenv;
use subxt::{Config, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./src/common/utils/polkadot_metadata_full.scale")]
pub mod polkadot {}
    
pub fn start() -> String{
    "Hello, world!".to_string()
}