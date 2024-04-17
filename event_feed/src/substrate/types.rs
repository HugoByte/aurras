use super::*;
use crate::common::ChainConfig;

pub struct PolkadotFeed<T: subxt::Config> {
    chain_config: ChainConfig,

    // Substrate client
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

impl<T: subxt::Config> PolkadotFeed<T> {


    /// The `event_feed` function in the code snippet is a method of the `PolkadotFeed` struct. It is an
    /// asynchronous function that takes a callback function `cb` as a parameter. This function
    /// subscribes to finalized blocks on the Polkadot chain and processes the extrinsics and events
    /// within those blocks.
    pub async fn event_feed(&self, cb: fn(events: Vec<serde_json::Value>)) {
        let mut blocks_sub = self.client.blocks().subscribe_finalized().await.unwrap();

        // For each block, print a bunch of information about it:
        loop {
            if let Some(block) = blocks_sub.next().await {
                let block = block.unwrap();
                let mut fetched_events = Vec::new();
                let extrinsics = block.extrinsics().await.unwrap();
                for ext in extrinsics.iter() {
                    let ext = ext.unwrap();
                    let events = ext.events().await.unwrap();
                    let event_details = events.iter().collect::<Result<Vec<_>, _>>().unwrap();

                    for event in event_details.iter() {
                        let s = event.field_values().unwrap();
                        let data = format!("{}", s);
                        let decode_event = Events {
                            pallet: event.pallet_name().to_string(),
                            method: event.variant_name().to_string(),
                            field_value: data,
                        };
                        let serialize_event = serde_json::to_value(&decode_event);
                        fetched_events.push(serialize_event.unwrap());
                    }
                }
                // Returning the events by callback function
                cb(fetched_events);
            }
        }
    }
}

#[macro_export]
macro_rules! events {
    ($exp: expr, $pallet:ident, $method:ident ) => {
        let transfer_event = $exp
            .find::<polkadot::$pallet::events::$method>()
            .collect::<Result<Vec<_>, _>>();
    };
}
