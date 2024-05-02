pub mod constants;
pub use constants::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod kuska_client;
pub use kuska_client::*;
mod context;
pub use context::*;

pub use super::*;

/// Represents the event feed chain configuration. This is used to configure the chain,
/// node endpoint and the filters. 
#[derive(Deserialize, Debug, Clone)]
#[derive(PartialEq)]
pub struct ChainConfig {
    pub chain: String,
    pub node_url: String,
    #[serde(default)]
    pub event_filter: String,
    #[serde(default)]
    pub contract_filter: String,
}

/// Represents the aurras ssb producer configuration. This is used to configure the producer
/// ip, port secret path and the invite key for the ssb pub. 
#[derive(Deserialize, Debug, Clone)]
pub struct ProducerConfig {
    #[serde(default = "default_host")]
    pub ssb_ip: String,
    #[serde(default = "default_port")]
    pub ssb_port: String,
    #[serde(default = "default_secret")]
    pub ssb_secret: String,
    #[serde(default)]
    pub ssb_invite: String,
}

/// Represents a single event in the event feed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeederEvent {
    pub name: String,
    pub contract: String,
    pub data: Value,
    pub block_number: usize,
    pub tx_hash: Option<String>,
    pub log_index: usize,
}
