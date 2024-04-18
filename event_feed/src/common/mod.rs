pub mod constants;
pub use constants::*;
use serde_json::Value;
use serde::Deserialize;


pub trait EventFeeder{
    fn event_feed(&self, cb: fn(events: Vec<serde_json::Value>));
}

#[derive(Deserialize, Debug)]
pub struct ChainConfig{
    pub node_url: String,
    #[serde(default)]
    pub filter : String,
}

#[derive(Deserialize, Debug)]
pub struct ProducerConfig{

    #[serde(default = "default_host")]
    pub ssb_ip: String,
    #[serde(default = "default_port" )]
    pub ssb_port: String,
    #[serde(default = "default_secret")]
    pub ssb_secret: String,
}