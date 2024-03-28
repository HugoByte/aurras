use super::*;

use serde::{Deserialize, Serialize};

pub struct Client {
    pub api: ApiCaller<TcpStream>,
    pub rpc_reader: RpcReader<TcpStream>,
    pub sk: SecretKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "type")]
    pub types: String,
    pub text: String,
    // mentions: Option<Vec<Mention>>,
}
