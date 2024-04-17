use super::*;
use crate::common::ChainConfig;

pub struct PolkadotFeed<T: subxt::Config> {
    chain_config: ChainConfig,
    client: OnlineClient<T>,
}

impl<T: subxt::Config> PolkadotFeed<T> {
    pub async fn new(chain_config: ChainConfig) -> PolkadotFeed<PolkadotConfig> {
        let client = OnlineClient::<PolkadotConfig>::from_url(&chain_config.node_url)
            .await
            .unwrap();
        PolkadotFeed {
            chain_config,
            client,
        }
    }
}

impl<T: subxt::Config> crate::common::EventFeeder for PolkadotFeed<T> {
    async fn event_feed(&self) -> serde_json::Value {
        let mut blocks_sub = self.client.blocks().subscribe_finalized().await.unwrap();
        let events = Vec::<serde_json::Value>::new();
        // For each block, print a bunch of information about it:
        loop {
            if let Some(block) = blocks_sub.next().await {
                let block = block.unwrap();

                let extrinsics = block.extrinsics().await.unwrap();
                for ext in extrinsics.iter() {
                    let ext = ext.unwrap();
                    let events = ext.events().await.unwrap();

                    println!("Static event details:");
                    for event in events.iter() {
                        let event = event.unwrap();

                        if let Ok(ev) = event.as_root_event::<polkadot::Event>() {
                            println!("{:?}", ev);
                        } else {
                            println!("<Cannot decode event>");
                        }
                    }
                }
            }
        }

        serde_json::json!({})
    }
}

#[macro_export]
macro_rules! events {
    ($exp: expr, $pallet:ident, $method:ident ) => {
        let transfer_event = 
        $exp.find::<polkadot::$pallet::events::$method>().collect::<Result<Vec<_>, _>>();
      
    };
}
