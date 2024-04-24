use super::*;
use crate::common;
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

            let filtered_events = Self::filter_event(&self, &events, client.clone()).await?;

            let feeder_events = filtered_events
                .into_iter()
                .map(|tendermint_event| {
                    let cloned_data = tendermint_event.data.clone();
                    let block = match cloned_data {
                        EventData::LegacyNewBlock {
                            ref block,
                            result_begin_block: _,
                            result_end_block: _,
                        } => block.as_ref().unwrap().header.version.block,
                        _ => unreachable!(),
                    };

                    let block_nmbr = block as usize;

                    let event_name = match cloned_data {
                        EventData::LegacyNewBlock {
                            block,
                            result_begin_block,
                            result_end_block: _,
                        } => {
                            let events = result_begin_block.unwrap().events;
                            let first_event = events.get(0);
                            match first_event {
                                Some(event) => {
                                    let kind = event.kind.to_string();
                                    kind
                                }
                                None => {
                                    println!("No events found in BeginBlock");
                                    String::default()
                                }
                            }
                        }
                        _ => unreachable!(),
                    };
                    common::FeederEvent {
                        name: event_name,
                        contract: String::default(),
                        data: match tendermint_event.data {
                            EventData::LegacyNewBlock {
                                block,
                                result_begin_block,
                                result_end_block,
                            } => serde_json::Value::String(
                                String::from_utf8_lossy(&block.unwrap().data[0]).to_string(),
                            ),
                            _ => serde_json::Value::String(String::default()),
                        },
                        block_number: block_nmbr,
                        tx_hash: Some(String::default()),
                        log_index: 0,
                    }
                })
                .collect::<Vec<common::FeederEvent>>();

            // Call the callback function with the filtered FeederEvents
            cb(feeder_events);
        }

        drop(subs);
        let _ = driver_handle.await; 
        Ok(())
    }

    fn convert_to_event(dialect_event: DialectEvent) -> Event {
        // Manually initialize Event
        let mut event = Event {
            query: Default::default(),
            data: EventData::GenericJsonEvent(().into()),
            events: Default::default(),
        };
        event
    }

    pub async fn filter_event(
        &self,
        event_name: &Vec<Event>,
        client: WebSocketClient,
    ) -> Result<Vec<Event>, Error> {
        let height = client.latest_block_results().await.unwrap().height;
        let mut filtered_events = Vec::new();

        loop {
            let query = Query::from(EventType::Tx).and_gte(height, 1_u64);

            let txs = match client.tx_search(query, false, 1, 1, Ascending).await {
                Ok(txs) => txs,
                Err(err) => {
                    eprintln!("Error searching for transactions: {}", err);
                    // break;
                    continue;
                }
            };

            for tx in txs.txs.iter() {
                let tx_data = tx.hash;

                let events_batch = match client.tx(tx_data, true).await {
                    Ok(events_batch) => events_batch.tx_result.events,
                    Err(err) => {
                        eprintln!("Error retrieving events for transaction: {}", err);
                        continue;
                    }
                };

                let matching_event = events_batch
                    .into_iter()
                    .filter_map(|event| {
                        // Convert each dialect event to the appropriate event type
                        Some(Self::convert_to_event(event))
                    })
                    .collect::<Vec<Event>>();

                println!("Matching events for TX {}: {:?}", tx_data, matching_event);

                filtered_events.extend(matching_event);
            }

            if !filtered_events.is_empty() {
                println!("Filtered events: {:?}", filtered_events);
                return Ok(filtered_events);
            }

            sleep(Duration::from_secs(1));
        }
    }
}
