use anyhow::{anyhow, Result};
use feeder::IconFeed;
use icon_sdk::icon_service;
use icon_sdk::icon_service::IconService;
use kuska_ssb::{api::dto::content::Mention, keystore::read_patchwork_config};
use runtime::{
    constants::{DEFAULT_PRODUCER_IP, DEFAULT_PRODUCER_PORT},
    kuska_ssb_client::client::Client,
    logger::{CoreLogger, Logger},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, thread::sleep, time::Duration};

mod feeder;
mod filter;
mod producer;
#[cfg(test)]
mod tests;
mod types;
mod utils;

pub use filter::*;
pub use producer::*;
pub use types::*;
pub use utils::*;

pub async fn start() -> Result<()> {
    dotenv::dotenv().ok();

    let ssb_ip = env::var("PRODUCER_IP").ok();
    let ssb_port = env::var("PRODUCER_PORT").ok();
    let ssb_secret = env::var("PRODUCER_SECRET").ok();
    let producer_invite = env::var("PRODUCER_INVITE").expect("invite address must be provided");
    let icon_api_url = env::var("ICON_API_URL")?;

    let events = get_list_env_var("FILTER_EVENTS");
    let score = get_list_env_var("FILTER_SCORE");

    let logger = CoreLogger::new(Some("./icon-feed.log"));

    logger.info("Connecting to producer...");

    let mut producer_client = Producer::create_ssb_producer(
        ssb_ip.as_deref(),
        ssb_port.as_deref(),
        ssb_secret.as_deref(),
    )
    .await?;

    producer_client.accept_invite(&producer_invite).await?;

    logger.info("Producer successfully connected✅");

    let filter = Filter { events, score };
    let mut icon_feed = IconFeed::new(&icon_api_url, producer_client, Box::new(logger)).await?;
    let invite = get_ssb_pub_address_from_invite(&producer_invite)?;
    icon_feed.get_event(filter, &invite).await?;

    Ok(())
}
