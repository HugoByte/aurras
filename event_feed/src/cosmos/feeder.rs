use super::*;
use crate::{common, cosmos};
use anyhow::*;
use tendermint_rpc::event::{Event, EventData};

/// The `CosmosFeed` struct in Rust defines a feed for handling Cosmos blockchain events with filtering
/// capabilities.
///
/// Properties:
///
/// * `chain_config`: The `chain_config` property in the `CosmosFeed` struct represents the
/// configuration settings for a specific blockchain network. It includes parameters such as the node
/// URL, event filters, and other relevant details needed to interact with the blockchain network.
/// * `events`: The `events` property in the `CosmosFeed` struct is a vector of strings that stores the
/// event filters for the Cosmos blockchain feed. These event filters are used to specify which types of
/// events the feed should listen to and process. The events are parsed from the `event_filter` string
/// provided
pub struct CosmosFeed {
    pub chain_config: ChainConfig,
    pub events: Vec<String>,
}

/// The above Rust code defines an implementation for a `CosmosFeed` struct. Here is a breakdown of what
/// the code is doing:
impl CosmosFeed {
    /// The function `new` creates a new `CosmosFeed` instance based on the provided `ChainConfig` with
    /// event filtering.
    ///
    /// Arguments:
    ///
    /// * `chain_config`: The `chain_config` parameter is of type `common::ChainConfig` and is used to
    /// configure the chain settings for the CosmosFeed struct.
    ///
    /// Returns:
    ///
    /// A `CosmosFeed` struct instance is being returned.
    pub fn new(chain_config: common::ChainConfig) -> CosmosFeed {
        let events = chain_config
            .event_filter
            .split(',')
            .map(|e| e.to_string())
            .filter(|e| !e.is_empty())
            .collect::<Vec<String>>();
        let cosmos = CosmosFeed {
            chain_config,
            events,
        };
        cosmos
    }

    /// The function `event_feed` in Rust asynchronously subscribes to new block events, processes and
    /// filters the events based on certain conditions, and calls a callback function with the filtered
    /// events.
    ///
    /// Arguments:
    ///
    /// * `cb`: The `cb` parameter in the `event_feed` function is a callback function that takes a
    /// vector of `serde_json::Value` as input. This callback function is called with the events
    /// received from the WebSocket subscription either directly or after filtering based on the event
    /// filter specified in the `chain_config`.
    ///
    /// Returns:
    ///
    /// The `event_feed` function returns a `Result<()>`.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        let (client, driver) = WebSocketClient::new(&*self.chain_config.node_url)
            .await
            .unwrap();
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let mut subs = client.subscribe(EventType::NewBlock.into()).await.unwrap();
        let mut events: Vec<Event> = Vec::new();

        while let Some(res) = subs.next().await {
            let ev = res.unwrap();
            events.push(ev.clone());

            // Check if event filter is empty (listen to all events)
            if self.chain_config.event_filter.is_empty() {
                // Convert all events to Value and call the callback directly
                let filter_events = events
                    .iter()
                    .map(|tendermint_event| {
                        let cloned_data = tendermint_event.data.clone();
                        match cloned_data {
                            EventData::LegacyNewBlock {
                                ref block,
                                result_begin_block,
                                result_end_block: _,
                            } => {
                                let block = block.as_ref().unwrap();
                                let block_number = block.header.version.block as usize;
                                let hash_string =
                                    block.header.last_commit_hash.map(|h| h.to_string());

                                // Similar to filtered events, construct FeederEvents with details
                                let events = result_begin_block
                                    .unwrap()
                                    .events
                                    .iter()
                                    .map(|event| common::FeederEvent {
                                        name: event.kind.to_string(),
                                        contract: String::default(),
                                        data: serde_json::Value::Null,
                                        block_number,
                                        tx_hash: hash_string.clone(),
                                        log_index: 0,
                                    })
                                    .map(|e| serde_json::to_value(e).unwrap()) // Convert to Value directly
                                    .collect::<Vec<serde_json::Value>>();
                                // Return the converted events directly
                                return events;
                            }
                            _ => unreachable!(),
                        }
                    })
                    .flatten()
                    .collect::<Vec<serde_json::Value>>();
                // Call the callback with all events
                cb(filter_events);
            } else {
                // logic for handling filtered events
                let filter_events = events
                    .clone()
                    .into_iter()
                    .filter_map(|tendermint_event| {
                        let cloned_data = tendermint_event.data.clone();
                        match cloned_data {
                            EventData::LegacyNewBlock {
                                ref block,
                                result_begin_block,
                                result_end_block: _,
                            } => {
                                let block = block.as_ref().unwrap();
                                let block_number = block.header.version.block as usize;
                                let hash_string =
                                    block.header.last_commit_hash.map(|h| h.to_string());

                                let filtered_events = result_begin_block
                                    .unwrap()
                                    .events
                                    .iter()
                                    .filter(|event| event.kind == self.chain_config.event_filter)
                                    .map(|event| common::FeederEvent {
                                        name: event.kind.to_string(),
                                        contract: String::default(),
                                        data: serde_json::Value::Null,
                                        block_number,
                                        tx_hash: hash_string.clone(),
                                        log_index: 0,
                                    })
                                    .collect::<Vec<common::FeederEvent>>();

                                if !filtered_events.is_empty() {
                                    Some(filtered_events)
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    })
                    .flatten();

                // Call the callback function with the filtered events
                cb(filter_events
                    .into_iter()
                    .map(|e| serde_json::to_value(e).unwrap())
                    .collect());
            };
        }

        drop(subs);
        let _ = driver_handle.await;
        Ok(())
    }
}
