use super::*;
// use crate::error::EventFeedError;
use anyhow::*;
use core::result::Result::Ok;

pub struct EthFeed {
    eth_service: Provider<Ws>,
    events: Vec<(String, H256)>,
    contracts: Vec<String>,
}

impl EthFeed {
    pub async fn new(config: ChainConfig) -> Result<EthFeed> {
        let events = config
            .event_filter
            .split(';')
            .filter(|evt| !evt.is_empty())
            .map(|evt| (evt.to_string(), H256::from(keccak256(evt))))
            .collect::<Vec<(String, H256)>>();

        let contracts = config
            .contract_filter
            .split(',')
            .map(|evt| evt.to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let client = Provider::<Ws>::connect(config.node_url).await?;

        let eth_feed = EthFeed {
            eth_service: client,
            events,
            contracts,
        };

        Ok(eth_feed)
    }

    pub async fn event_feed(&self, cb: &dyn Fn(Vec<Value>)) -> Result<()> {
        let client = Arc::new(&self.eth_service);
        
        let last_block = match client.get_block(BlockNumber::Latest).await? {
            Some(block) => block.number.unwrap(),
            None => return Err(anyhow!("Failed to fetch the latest block number")),
        };


        let events = self.events.iter().map(|e| e.0.clone());

        let filter = Filter::new().from_block(last_block - 25).events(events);

        let mut stream = match client.subscribe_logs(&filter).await {
            Ok(stream) => stream,
            Err(e) => return Err(anyhow!("Failed to subscribe to logs: {}", e)),
        };

        while let Some(log) = stream.next().await {
            if self.contracts.is_empty() || self.contracts.contains(&format!("{:?}", &log.address))
            {
                let tx_hash = match log.transaction_hash {
                    Some(hash) => hash,
                    None => {
                        let event = self.events.iter().find(|e| e.1 == log.topics[0]);

                        cb(vec![serde_json::to_value(FeederEvent {
                            name: event.unwrap().0.clone(),
                            contract: format!("{:?}", &log.address),
                            data: json!({ "topics": log.topics, "data": log.data.to_vec() }),
                            block_number: log.block_number.unwrap_or_default().as_usize(),
                            tx_hash: None,
                            log_index: log.log_index.unwrap_or_default().as_usize(),
                        })
                        .unwrap()]);
                        continue;
                    }
                };

                let tx_receipt = client.get_transaction_receipt(tx_hash).await;

                if let Ok(Some(tx_receipt)) = tx_receipt {
                    let mut logs = Vec::<Value>::new();

                    for log in tx_receipt.logs.iter() {
                        if self.contracts.is_empty()
                            || self.contracts.contains(&format!("{:?}", &log.address))
                        {
                            for evt in self.events.iter() {
                                if log.topics[0] == evt.1 {
                                    logs.push(
                                        serde_json::to_value(
                                        FeederEvent {
                                        name: evt.0.clone(),
                                        contract: format!("{:?}", &log.address),
                                        data: json!({ "topics": log.topics, "data": log.data.to_vec() }),
                                        block_number: log.block_number.unwrap_or_default().as_usize(),
                                        tx_hash: Some(format!("{:?}", tx_hash)),
                                        log_index: log.log_index.unwrap_or_default().as_usize(),
                                        }).unwrap()
                                    );
                                    break;
                                }
                            }
                        }
                    }

                    if !logs.is_empty() {
                        cb(logs);
                    }
                }
            }
        }
        

        Ok(())
    }
}
