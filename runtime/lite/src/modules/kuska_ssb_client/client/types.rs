use super::*;

use kuska_ssb::api::dto::content::Mention;
use serde::{Deserialize, Serialize};

pub struct Client {
    pub api: ApiCaller<TcpStream>,
    pub rpc_reader: RpcReader<TcpStream>,
    pub sk: SecretKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event: String,
    pub section: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
}

impl Event {
    pub fn new(
        event: String,
        section: String,
        content: String,
        mentions: Option<Vec<Mention>>,
    ) -> Self {
        Self {
            event,
            section,
            content,
            mentions,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "type")]
    pub types: String,
    pub text: String,
    // mentions: Option<Vec<Mention>>,
}
