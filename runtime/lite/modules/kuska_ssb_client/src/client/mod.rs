extern crate kuska_handshake;
extern crate kuska_ssb;

extern crate base64;
extern crate crossbeam;
extern crate regex;
extern crate structopt;

use std::fmt::Debug;

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
    feed::{is_privatebox, privatebox_decipher, Feed, Message},
    keystore::{from_patchwork_local, OwnedIdentity},
    rpc::{RecvMsg, RequestNo, RpcReader, RpcWriter},
};

mod client;
mod errors;
mod response_parser;

pub use client::*;
pub use errors::*;
pub use response_parser::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
