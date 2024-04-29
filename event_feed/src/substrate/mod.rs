#[cfg(test)]
mod tests;
pub mod types;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./src/common/utils/polkadot_metadata_full.scale")]
pub mod polkadot {}
/// The `Events` struct in Rust contains fields for pallet, method, and field value.
///
/// Properties:
///
/// * `pallet`: The `pallet` property in the `Events` struct represents the name of the pallet/module
/// where the event is defined.
/// * `method`: The `method` property in the `Events` struct represents the method associated with the
/// event. It is a String type field.
/// * `field_value`: The `field_value` property in the `Events` struct represents a string value
/// associated with a specific event. It could be used to store additional information or data related
/// to the event being described by the `pallet` and `method` properties.

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pallet: String,
    method: String,
    field_value: String,
}
