use super::*;

use serde::{Deserialize, Serialize};

/// Represents an SSB client.
pub struct Client {
    pub api: ApiCaller<TcpStream>,
    pub rpc_reader: RpcReader<TcpStream>,
    pub sk: SecretKey,
}

/// Represents the content of an SSB feed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "type")]
    pub types: String,
    pub text: String,
}
