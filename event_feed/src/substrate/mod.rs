pub mod types;

use dotenv::dotenv;
use subxt::{Config, OnlineClient, PolkadotConfig};
use serde::{Serialize, Deserialize};

#[subxt::subxt(runtime_metadata_path = "./src/common/utils/polkadot_metadata_full.scale")]
pub mod polkadot {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Events{
    pallet: String,
    method: String,
    field_value: String,
}