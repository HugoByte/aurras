use ethers::types::BlockId::Hash;
use std::any::Any;

use super::*;
use crate::substrate::polkadot::offences::events::offence::Kind;
use crate::{common, cosmos};
use tendermint_rpc::dialect::v0_37::Event as DialectEvent;
use tendermint_rpc::event::{Event, EventData};

pub struct CosmosFeed {
    pub chain_config: ChainConfig,
    pub events: Vec<String>,
}

impl CosmosFeed {
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

    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<(), Error> {
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
