use crate::common::ChainConfig;
use futures::StreamExt;
use tendermint_rpc::query::EventType;
use tendermint_rpc::{SubscriptionClient, WebSocketClient};

mod feeder;
pub use feeder::*;

#[cfg(test)]
mod tests;
