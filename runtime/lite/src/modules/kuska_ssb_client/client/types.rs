use super::*;

use serde::{Deserialize, Serialize};

/// Represents an SSB client.
/// 
/// # Properties
/// 
/// * `api`: An instance of the `ApiCaller` struct that is used to make RPC calls to the SSB server.
/// It is of type `ApiCaller<TcpStream>`, where `TcpStream` is the type of the stream used to communicate
/// with the SSB server.
/// * `rpc_reader`: An instance of the `RpcReader` struct that is used to read messages from the
/// SSB server. It is of type `RpcReader<TcpStream>`, where `TcpStream` is the type of the stream used
/// to communicate with the SSB server.
/// * `sk`: An instance of the `SecretKey` struct that represents the secret key associated with the
/// SSB client.
pub struct Client {
    pub api: ApiCaller<TcpStream>,
    pub rpc_reader: RpcReader<TcpStream>,
    pub sk: SecretKey,
}

/// Represents the content of an SSB feed.
/// 
/// # Properties
/// 
/// * `types`: A string slice that represents the type of the content.
/// * `text`: A string slice that represents the text of the content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "type")]
    pub types: String,
    pub text: String,
}
