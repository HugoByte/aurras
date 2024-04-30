use crate::common::ChainConfig;
use futures::StreamExt;
use std::fmt::Error;
use std::thread::sleep;
use std::time::Duration;
use tendermint_rpc::event::Event;
use tendermint_rpc::query::{EventType, Query};
use tendermint_rpc::Order::Ascending;
use tendermint_rpc::{Client, SubscriptionClient, WebSocketClient};

mod feeder;
pub use feeder::*;

#[cfg(test)]
mod tests;
