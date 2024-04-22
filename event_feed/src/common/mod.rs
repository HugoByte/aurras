pub mod constants;
pub use constants::*;
use serde::Deserialize;
use serde_json::Value;

pub trait EventFeeder {
    fn event_feed(&self) -> impl std::future::Future<Output = Value> + Send;
}

#[derive(Deserialize, Debug, Clone)]
#[derive(PartialEq)]
pub struct ChainConfig {
    pub node_url: String,
    #[serde(default)]
    pub event_filter: String,
    #[serde(default)]
    pub contract_filter: String,
}

#[derive(Deserialize, Debug)]
pub struct ProducerConfig {
    #[serde(default = "default_host")]
    pub ssb_ip: String,
    #[serde(default = "default_port")]
    pub ssb_port: String,
    #[serde(default = "default_secret")]
    pub ssb_secret: String,
}
