use super::*;
use crate::common;

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

    pub async fn event_feed(chain_config: ChainConfig) -> Result<Vec<Event>, Error> {
        let (client, driver) = WebSocketClient::new(&*chain_config.node_url).await.unwrap();
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let mut subs = client.subscribe(EventType::NewBlock.into()).await.unwrap();
        let mut events: Vec<Event> = Vec::new();

        while let Some(res) = subs.next().await {
            let ev = res.unwrap();
            events.push(ev.clone());

            println!("Event for latest block: {:?}", ev);
        }

        drop(subs);
        let _ = driver_handle.await; // Wait for driver to finish
        Ok(events)
    }

    pub async fn filter_event(
        event_name: &[&str],
        client: WebSocketClient,
        chain_config: ChainConfig,
    ) -> Result<(), Error> {
        let mut events = match Self::event_feed(chain_config).await {
            Ok(events) => events,
            Err(err) => {
                eprintln!("Error getting events: {}", err);
                return Err(err);
            }
        };
        println!("{:?}", events);

        let height = client.latest_block_results().await.unwrap().height;

        loop {
            let mut filtered_events = Vec::new();
            let query = Query::from(EventType::Tx).and_gte(height, 1_u64);

            let txs = match client.tx_search(query, false, 1, 1, Ascending).await {
                Ok(txs) => txs,
                Err(err) => {
                    eprintln!("Error searching for transactions: {}", err);
                    break;
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

                let matching_events = events_batch
                    .into_iter()
                    .filter(|event| event_name.iter().any(|name| name == &event.kind))
                    .collect::<Vec<_>>();

                println!("Matching events for TX {}: {:?}", tx_data, matching_events);

                filtered_events.extend(matching_events);
            }

            if !filtered_events.is_empty() {
                println!("Filtered events: {:?}", filtered_events);
            }

            sleep(Duration::from_secs(1));
        }
        Ok(())
    }
}
