pub mod constants;
pub use constants::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod kuska_client;
pub use kuska_client::*;
mod context;
pub use context::*;

pub use super::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ChainConfig {
    #[serde(default)]
    pub chain: String,
    #[serde(default = "default_endpoint")]
    pub node_url: String,
    #[serde(default)]
    pub event_filter: String,
    #[serde(default)]
    pub contract_filter: String,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeederEvent {
    pub name: String,
    pub contract: String,
    pub data: Value,
    pub block_number: usize,
    pub tx_hash: Option<String>,
    pub log_index: usize,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain: Default::default(),
            node_url: "ws://127.0.0.1:9944".to_string(),
            event_filter: Default::default(),
            contract_filter: Default::default(),
        }
    }
}

impl Default for ProducerConfig {
    fn default() -> Self {
        Self {
            ssb_ip: default_host(),
            ssb_port: default_port(),
            ssb_secret: default_secret(),
            ssb_invite: Default::default(),
        }
    }
}
