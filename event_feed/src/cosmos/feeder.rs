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

    pub async fn event_feed(&self, cb: &dyn Fn(Vec<common::FeederEvent>)) -> Result<(), Error> {
        let (client, driver) = WebSocketClient::new(&*self.chain_config.node_url)
            .await
            .unwrap();
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let mut subs = client.subscribe(EventType::NewBlock.into()).await.unwrap();
        let mut events: Vec<Event> = Vec::new();

        while let Some(res) = subs.next().await {
            let ev = res.unwrap();
            events.push(ev.clone());
    
            let filter_events = events
                .clone()
                .into_iter()
                .map(|tendermint_event| {
                    let cloned_data = tendermint_event.data.clone();
                    let (block, event_kinds, hash_string) = match cloned_data {
                        EventData::LegacyNewBlock {
                            ref block,
                            result_begin_block,
                            result_end_block: _,
                        } => {
                            let b = block.as_ref().unwrap();
                            let block = b.header.version.block;
                            let hash = b.header.last_commit_hash;

                            let hash_string = hash.map(|hash| hash.to_string());

                            let event_kinds = result_begin_block
                                .unwrap()
                                .events
                                .iter()
                                .filter(|event| event.kind == event.kind)
                                .map(|event| event.kind.to_string())
                                .collect::<Vec<String>>()
                                .join(",");

                            // let data: serde_json::Value =
                            //     serde_json::to_value(&cloned_data.clone()).unwrap();

                            (block as usize, event_kinds, hash_string)
                        }

                        _ => unreachable!(),
                    };

                    common::FeederEvent {
                        name: event_kinds,
                        contract: String::default(),
                        data: serde_json::Value::Null,
                        block_number: block,
                        tx_hash: hash_string,
                        log_index: 0,
                    }
                })
                .collect::<Vec<common::FeederEvent>>();

            // Call the callback function with the filtered FeederEvents
            cb(filter_events);
        }

        drop(subs);
        let _ = driver_handle.await;
        Ok(())
    }
}
