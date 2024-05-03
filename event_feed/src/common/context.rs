use super::CosmosFeed;
use super::EthFeed;
use crate::IconFeed;
use crate::PolkadotFeed;
// use crate::error::EventFeedError;
use anyhow::Result;
use subxt::PolkadotConfig;

pub enum Context {
    PolkadotFeed(PolkadotFeed<PolkadotConfig>),
    IconFeed(IconFeed),
    EthFeed(EthFeed),
    CosmosFeed(CosmosFeed),
}

impl Context {
    pub async fn feed_events(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        match self {
            Context::PolkadotFeed(feed) => feed.event_feed(cb).await,
            Context::IconFeed(feed) => feed.event_feed(cb).await,
            Context::EthFeed(feed) => feed.event_feed(cb).await,
            Context::CosmosFeed(feed) => feed.event_feed(cb).await,
        }
    }
}
