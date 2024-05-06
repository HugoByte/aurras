use super::*;
use crate::common::ChainConfig;
use crate::error::IOError;
use anyhow::*;
use core::result::Result::Ok;

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

    /// Creates a new `PolkadotFeed` instance based on the provided chain configuration.
    /// 
    /// # Arguments
    /// 
    /// * `chain_config`: An event feed chain configuration.
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
    /// Fetches the events from the Substrate chain and returns it through the provided callback function.
    /// 
    /// # Arguments
    /// 
    /// * `cb`: A callback function to return the events to the caller.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<(), IOError>{
        let mut blocks_sub = self.client.blocks().subscribe_finalized().await.map_err(|err| IOError::Subxt(err.into()))?;

        // For each block, print a bunch of information about it:
        loop {
            if let Some(block) = blocks_sub.next().await {
                let block = block.map_err(|err| IOError::Subxt(err.into()))?;
                let mut fetched_events = Vec::new();
                let extrinsics = block.extrinsics().await.map_err(|err| IOError::Subxt(err.into()))?;
                for ext in extrinsics.iter() {
                    let ext = match ext{
                        Ok(ext) => ext,
                        Err(err) => return Err(IOError::Subxt(err)),
                    };
                    let events = match ext.events().await{
                        Ok(events) => events,
                        Err(err) => return Err(IOError::Other(format!("Error fetching events: {}", err))),
                    };
                    let event_details = match events.iter().collect::<Result<Vec<_>, _>>(){
                        Ok(event_details) => event_details,
                        Err(err) => return Err(IOError::Other(format!("Error collecting events: {}", err))),
                    };

                    let filter = self.split_filter();

                    for event in event_details.iter() {
                        let s = match event.field_values(){
                            Ok(data) => data,
                            Err(err) => return Err(IOError::Other(format!("Error getting event field values: {}", err))),

                        };
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
                            let serialize_event = serde_json::to_value(&decode_event).map_err(|err| IOError::Other(format!("Serialising error {}", err)))?;
                            fetched_events.push(serialize_event);
                        }
                    }
                }
                // Returning the events by callback function
                cb(fetched_events);
            }
        }
    }

    /// Splits a string by ';' and then by '=' to create a HasMap. This hashmap is used to filter the events.
    /// 
    /// Returns:
    /// 
    /// Returns the hashmap with the key as the pallet name and the value as the list of events.
    pub fn split_filter(&self) -> std::collections::HashMap<String, Vec<&str>> {
        let mut filter_map: std::collections::HashMap<String, Vec<&str>> =
            std::collections::HashMap::new();
        if !self.chain_config.event_filter.is_empty() {
            for pair in self.chain_config.event_filter.split(';') {
                if !pair.contains('=') {
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

/// A macro to extract the list of particular event from the list of substrate events.
#[macro_export]
macro_rules! events {
    ($exp: expr, $pallet:ident, $method:ident ) => {
        let transfer_event = $exp
            .find::<polkadot::$pallet::events::$method>()
            .collect::<Result<Vec<_>, _>>();
    };
}
