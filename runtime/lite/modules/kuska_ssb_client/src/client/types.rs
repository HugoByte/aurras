use serde::{Deserialize, Serialize};

use super::*;

pub struct Client {
    pub api: ApiCaller<TcpStream>,
    pub rpc_reader: RpcReader<TcpStream>,
    pub sk: SecretKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub body: String,
}

pub struct UserConfig {
    pub public_key: String,
    pub secret_key: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "type")]
    types: String,
    pub text: String,
}

impl UserConfig {
    pub fn new(public_key: &str, secret_key: &str, id: &str) -> Self {
        Self {
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
            id: id.to_string(),
        }
    }
}
