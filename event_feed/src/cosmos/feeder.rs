use super::*;
use crate::common;
use crate::error::IOError;
use anyhow::*;
use core::result::Result::Ok;
use serde_json::Value;
use tendermint_rpc::event::{Event, EventData};

/// Represents the cosmos blockchain event feed.
pub struct CosmosFeed {
    pub chain_config: ChainConfig,
    pub events: Vec<String>,
}

impl CosmosFeed {
    /// Creates a new CosmosFeed instance based on the provided chain configuration.
    ///
    /// Arguments:
    ///
    /// * `chain_config`: An event feed chain configuration.
    pub fn new(chain_config: common::ChainConfig) -> CosmosFeed {
        let events = chain_config
            .event_filter
            .split(',')
            .map(|e| e.to_string())
            .filter(|e| !e.is_empty())
            .collect::<Vec<String>>();
        CosmosFeed {
            chain_config,
            events,
        }
    }

    /// Fetches the events from the Cosmos chain and returns it through the provided callback function.
    ///
    /// Arguments:
    ///
    /// * `cb`: A callback function to return the events to the caller.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        let (client, driver) = WebSocketClient::new(&*self.chain_config.node_url)
            .await
            .map_err(|err| IOError::Other(format!("Client not created {}", err)))?;
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let mut subs = client
            .subscribe(EventType::NewBlock.into())
            .await
            .map_err(|err| IOError::Other(format!("Error in subscribing the client {}", err)))?;
        let mut events: Vec<Event> = Vec::new();

        while let Some(res) = subs.next().await {
            match res {
                Ok(ev) => {
                    events.push(ev.clone());
                    let filter_events = events
                        .iter()
                        .filter(|tendermint_event| {
                            Self::convert_to_feeder_event(tendermint_event, &self.chain_config)
                                .is_some()
                        })
                        .flat_map(|tendermint_event| {
                            Self::convert_to_feeder_event(tendermint_event, &self.chain_config)
                                .unwrap()
                        })
                        .collect::<Vec<serde_json::Value>>();
                    cb(filter_events);
                    events.clear();
                }
                Err(err) => {
                    // Consider retry logic or other actions here
                    return Err(IOError::Other(format!("Error receving event {} ", err)).into_anyhow());
                }
            }
        }

        drop(subs);
        let _ = driver_handle.await;
        Ok(())
    }

    /// Converts a cosmos Tendermint event to a list of json `FeederEvent` objects.
    ///
    /// Arguments:
    ///
    /// * `event`: A tendermint event.
    /// * `chain_config`: The event feed chain configuration.
    ///
    /// Returns:
    ///
    /// Returns the list of `FeederEvent` objects as json values.
    fn convert_to_feeder_event(
        event: &Event,
        chain_config: &ChainConfig,
    ) -> Option<Vec<serde_json::Value>> {
        match event.data.clone() {
            EventData::LegacyNewBlock {
                ref block,
                result_begin_block,
                result_end_block: _,
            } => {
                let block = block.as_ref()?;
                let block_number = block.header.version.block as usize;
                let hash_string = block.header.last_commit_hash.map(|h| h.to_string());

                let filtered_events: Vec<Value> = if chain_config.event_filter.is_empty() {
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
                        .map(|e| {
                            serde_json::to_value(e).unwrap_or_else(|err| {
                                eprintln!("Error converting FeederEvent to Json: {}", err);
                                serde_json::Value::Null
                            })
                        })
                        .collect()
                } else {
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
                        .map(|e| {
                            serde_json::to_value(e).unwrap_or_else(|err| {
                                eprintln!("Error converting FeederEvent to Json: {}", err);
                                serde_json::Value::Null
                            })
                        })
                        .collect()
                };

                if !filtered_events.is_empty() {
                    Some(filtered_events)
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}
