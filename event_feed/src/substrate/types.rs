use super::*;
use crate::common::ChainConfig;

#[derive(Debug, Clone)]
/// The `PolkadotFeed` struct in Rust contains a `chain_config` field of type `ChainConfig` and a
/// `client` field of type `OnlineClient<T>`.
/// 
/// Properties:
/// 
/// * `chain_config`: The `chain_config` property is a field of type `ChainConfig` in the `PolkadotFeed`
/// struct. It likely contains configuration settings related to the Polkadot chain that the feed is
/// interacting with.
/// * `client`: The `client` property in the `PolkadotFeed` struct is an instance of the `OnlineClient`
/// struct parameterized with the type `T`, which must implement the `subxt::Config` trait. This client
/// is used to interact with the Substrate blockchain associated with the Polkad
pub struct PolkadotFeed<T: subxt::Config> {
    chain_config: ChainConfig,

    // Substrate client
    client: OnlineClient<T>,
}

impl<T: subxt::Config> PolkadotFeed<T> {
    pub async fn new(chain_config: ChainConfig) -> Result<PolkadotFeed<PolkadotConfig>> {
        let client = OnlineClient::<PolkadotConfig>::from_url(&chain_config.node_url)
            .await
            ?;
        Ok(PolkadotFeed {
            chain_config,
            client,
        })
    }
}

impl<T: subxt::Config> PolkadotFeed<T> {
    /// The `event_feed` function in the code snippet is a method of the `PolkadotFeed` struct. It is an
    /// asynchronous function that takes a callback function `cb` as a parameter. This function
    /// subscribes to finalized blocks on the Polkadot chain and processes the extrinsics and events
    /// within those blocks.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()>{
        let mut blocks_sub = self.client.blocks().subscribe_finalized().await?;

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

                    let filter = self.split_filter();

                    for event in event_details.iter() {
                        let s = event.field_values().unwrap();
                        let data = format!("{}", s).replace("((", "[").replace("))", "]");

                        let pallet_name = event.pallet_name().to_lowercase();
                        if !self.chain_config.event_filter.is_empty() {
                            if filter.contains_key(pallet_name.as_str()) {
                                if let Some(values) = filter.get(pallet_name.as_str()) {
                                    if values.contains(&event.variant_name()) || values.is_empty() {
                                        let decode_event = Events {
                                            pallet: event.pallet_name().to_string(),
                                            method: event.variant_name().to_string(),
                                            field_value: data,
                                        };
                                        let serialize_event = serde_json::to_value(&decode_event);
                                        fetched_events.push(serialize_event.unwrap());
                                    }
                                }
                            }
                        } else {
                            let decode_event = Events {
                                pallet: event.pallet_name().to_string(),
                                method: event.variant_name().to_string(),
                                field_value: data,
                            };
                            let serialize_event = serde_json::to_value(&decode_event)?;
                            fetched_events.push(serialize_event);
                        }
                    }
                }
                // Returning the events by callback function
                cb(fetched_events);
            }
        }
    }

    /// The function `split_filter` in Rust splits a string by ';' and then by '=' to create a HashMap
    /// with keys and corresponding values as vectors.
    /// 
    /// Returns:
    /// 
    /// A `HashMap` containing keys of type `String` and values of type `Vec<&str>` is being returned.
    pub fn split_filter(&self) -> std::collections::HashMap<String, Vec<&str>> {
        let mut filter_map: std::collections::HashMap<String, Vec<&str>> =
            std::collections::HashMap::new();
        if !self.chain_config.event_filter.is_empty() {
            for pair in self.chain_config.event_filter.split(';') {
                if !pair.contains("=") {
                    filter_map.insert(pair.to_string(), vec![]);
                }
                if pair.is_empty() {
                    continue;
                }

                let mut iter = pair.split('=');
                if let Some(key) = iter.next() {
                    if let Some(values) = iter.next() {
                        let values_vec: Vec<&str> = values.split(',').collect();
                        filter_map.insert(key.to_string(), values_vec);
                    }
                }
            }
        }
        filter_map
    }
}

/// The `events` macro defined with `macro_rules!` in Rust is used for generating code at compile time.
#[macro_export]
macro_rules! events {
    ($exp: expr, $pallet:ident, $method:ident ) => {
        let transfer_event = $exp
            .find::<polkadot::$pallet::events::$method>()
            .collect::<Result<Vec<_>, _>>();
    };
}
