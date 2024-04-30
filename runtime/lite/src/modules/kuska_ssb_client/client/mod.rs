extern crate kuska_handshake;
extern crate kuska_ssb;

extern crate base64;

use std::fmt::Debug;

use crate::AppError;
use async_std::net::TcpStream;
use kuska_handshake::async_std::{handshake_client, BoxStream};
use kuska_sodiumoxide::crypto::sign::{ed25519, SecretKey};
use kuska_ssb::api::dto::{
    content::TypedMessage, CreateHistoryStreamIn, CreateStreamIn, LatestOut, WhoAmIOut,
};
use kuska_ssb::{
    api::ApiCaller,
    crypto::ed25519::PublicKey,
    discovery::ssb_net_id,
    feed::{Feed, Message},
    keystore::{from_patchwork_local, OwnedIdentity},
    rpc::{RecvMsg, RequestNo, RpcReader, RpcWriter},
};

#[allow(clippy::module_inception)]
mod client;
mod feed_processor;
mod response_parser;

pub use client::*;
pub use feed_processor::*;
pub use response_parser::*;

pub mod types;
pub use types::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
pub mod tests;

/// Asynchronous function that a `OwnedIdentity` from a ssb `secret` file path
/// 
/// # Arguments
/// 
/// * `file_path` - A string slice representing the path to the ssb `secret` file
/// 
/// # Returns
/// 
/// Returns a `Result<OwnedIdentity>` that contains an instance of the `OwnedIdentity` struct or an error.
pub async fn owned_identity_from_patchwork_config(file_path: &str) -> Result<OwnedIdentity> {
    let mut file = async_std::fs::File::open(file_path).await?;
    Ok(kuska_ssb::keystore::read_patchwork_config(&mut file)
        .await
        .unwrap())
}
