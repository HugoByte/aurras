#[cfg(test)]
mod tests;
pub mod types;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./src/common/utils/polkadot_metadata_full.scale")]
pub mod polkadot {}
/// Represents the substrate blockchain event.
#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pallet: String,
    method: String,
    field_value: String,
}
