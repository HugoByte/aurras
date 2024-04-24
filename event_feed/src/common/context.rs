use crate::IconFeed;
use crate::PolkadotFeed;
use subxt::PolkadotConfig;
use anyhow::Result;

pub enum Context {
    PolkadotFeed(PolkadotFeed<PolkadotConfig>),
    IconFeed(IconFeed),
}

impl Context {
    pub async fn feed_events(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()>{
        match self {
            Context::PolkadotFeed(feed) => feed.event_feed(cb).await,
            Context::IconFeed(feed) => {
                feed.event_feed(cb).await
            }
        }
    }
}
