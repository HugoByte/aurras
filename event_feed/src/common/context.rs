use super::CosmosFeed;
use super::EthFeed;
// use crate::error::IOError;
use crate::IconFeed;
use crate::PolkadotFeed;
use subxt::PolkadotConfig;
use anyhow::Result;
/// Represents the main context of the event feed. This is used to configure the event feed with the
/// appropriate chain.
pub enum Context {
    PolkadotFeed(PolkadotFeed<PolkadotConfig>),
    IconFeed(IconFeed),
    EthFeed(EthFeed),
    CosmosFeed(CosmosFeed),
}

impl Context {
    /// Starts the event feed.
    pub async fn feed_events(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        match self {
            Context::PolkadotFeed(feed) => feed.event_feed(cb).await,
            Context::IconFeed(feed) => feed.event_feed(cb).await,
            Context::EthFeed(feed) => feed.event_feed(cb).await,
            Context::CosmosFeed(feed) => feed.event_feed(cb).await,
        }
    }
}
