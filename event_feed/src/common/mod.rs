pub mod constants;
pub use constants::*;
use serde::Deserialize;
use serde_json::Value;
mod kuska_client;
pub use kuska_client::*;
mod context;
pub use context::*;

pub use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct ChainConfig {
    pub chain: String,
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
