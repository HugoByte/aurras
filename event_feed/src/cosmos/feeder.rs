use super::*;
use crate::{common, cosmos};
use anyhow::*;
use runtime::{logger::CoreLogger, Logger};
use serde_json::Value;
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
    pub logger: CoreLogger,
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
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let cosmos = CosmosFeed {
            chain_config,
            events,
            logger,
        };
        cosmos
    }

    /// This Rust function asynchronously subscribes to a WebSocket event feed, filters and processes
    /// the events, and calls a callback function with the results.
    ///
    /// Arguments:
    ///
    /// * `cb`: The `cb` parameter in the `event_feed` function is a callback function that takes a
    /// vector of `serde_json::Value` as input. This callback function is called with the filtered
    /// events obtained from the WebSocket subscription loop.
    ///
    /// Returns:
    ///
    /// The `event_feed` function returns a `Result<()>`.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        let (client, driver) = WebSocketClient::new(&*self.chain_config.node_url)
            .await
            .unwrap();
        self.logger.info(&format!(
            "Following the publisher {}",
            self.chain_config.node_url
        ));
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let mut subs = client.subscribe(EventType::NewBlock.into()).await.unwrap();
        let mut events: Vec<Event> = Vec::new();

        while let Some(res) = subs.next().await {
            let ev = res.unwrap();
            events.push(ev.clone());

            let filter_events = events
                .iter()
                .filter(|tendermint_event| {
                    Self::convert_to_feeder_event(self, tendermint_event, &self.chain_config)
                        .is_some()
                })
                .flat_map(|tendermint_event| {
                    Self::convert_to_feeder_event(self, tendermint_event, &self.chain_config)
                        .unwrap()
                })
                .collect::<Vec<serde_json::Value>>();

            // Call the callback with all or filtered events
            cb(filter_events);

            events.clear();
        }

        drop(subs);
        self.logger.info("Websocket connection closed!!");
        let _ = driver_handle.await;
        Ok(())
    }

    /// The function `convert_to_feeder_event` takes an event and chain configuration as input, extracts
    /// relevant data from the event, filters events based on configuration, and returns a vector of
    /// JSON values representing feeder events.
    ///
    /// Arguments:
    ///
    /// * `event`: The `event` parameter is a reference to an `Event` struct.
    /// * `chain_config`: The `chain_config` parameter in the `convert_to_feeder_event` function is of
    /// type `&ChainConfig`, which likely contains configuration settings specific to the blockchain
    /// network or chain being used. This parameter is used to filter events based on certain criteria
    /// specified in the `chain_config` object.
    ///
    /// Returns:
    ///
    /// The function `convert_to_feeder_event` returns an `Option` containing a `Vec` of
    /// `serde_json::Value` elements.
    fn convert_to_feeder_event(
        &self,
        event: &Event,
        chain_config: &ChainConfig,
    ) -> Option<Vec<serde_json::Value>> {
        self.logger
            .debug(&format!("Processing events : {:?}", event));
        match event.data.clone() {
            EventData::LegacyNewBlock {
                ref block,
                result_begin_block,
                result_end_block: _,
            } => {
                let block = block.as_ref().unwrap();
                let block_number = block.header.version.block as usize;
                let hash_string = block.header.last_commit_hash.map(|h| h.to_string());
                self.logger.info(&format!(
                    "Processing LegacyNewBlockEvent for block: {}",
                    block.header.version.block
                ));

                let filtered_events: Vec<Value> = if chain_config.event_filter.is_empty() {
                    self.logger.info(&format!(
                        "Processing all events from block : {}",
                        block_number
                    ));
                    result_begin_block
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
                        .map(|e| serde_json::to_value(e).unwrap())
                        .collect()
                } else {
                    self.logger.info("Filtering events based on the event name");
                    result_begin_block
                        .unwrap()
                        .events
                        .iter()
                        .filter(|event| event.kind == chain_config.event_filter)
                        .map(|event| common::FeederEvent {
                            name: event.kind.to_string(),
                            contract: String::default(),
                            data: serde_json::Value::Null,
                            block_number,
                            tx_hash: hash_string.clone(),
                            log_index: 0,
                        })
                        .collect::<Vec<common::FeederEvent>>()
                        .into_iter()
                        .map(|e| serde_json::to_value(e).unwrap())
                        .collect()
                };

                if !filtered_events.is_empty() {
                    self.logger.info("returning the filtered events");
                    Some(filtered_events)
                } else {
                    self.logger.info("No events matched the filter");
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}
