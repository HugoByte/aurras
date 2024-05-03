use super::*;
/// Represents an Ethereum blockchain event feed.
pub struct EthFeed {
    eth_service: Provider<Ws>,
    events: Vec<(String, H256)>,
    contracts: Vec<String>,
}

impl EthFeed {
    /// Creates a new EthFeed instance based on the provided chain configuration.
    /// 
    /// Arguments:
    /// 
    /// * `config`: An event feed chain configuration.
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

    /// Fetches the events from the Ethereum chain and returns it through the provided callback function.
    /// 
    /// # Arguments:
    /// 
    /// * `cb`: A callback function to return the events to the caller.
    pub async fn event_feed(&self, cb: &dyn Fn(Vec<Value>)) -> Result<()> {
        let client = Arc::new(&self.eth_service);

        let last_block = client
            .get_block(BlockNumber::Latest)
            .await?
            .unwrap()
            .number
            .unwrap();

        let events = self.events.iter().map(|e| e.0.clone());

        let filter = Filter::new().from_block(last_block - 25).events(events);

        let mut stream = client.subscribe_logs(&filter).await?;

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
                            let mut filtered = false;
                            let mut event_name = "Unknown".to_string();

                            if self.events.is_empty() {
                                filtered = true;
                            } else {
                                for evt in self.events.iter() {
                                    if log.topics[0] == evt.1 {
                                        filtered = true;
                                        event_name = evt.0.clone();
                                        break;
                                    }
                                }
                            }

                            if filtered {
                                logs.push(
                                    serde_json::to_value(
                                FeederEvent {
                                name: event_name,
                                contract: format!("{:?}", &log.address),
                                data: json!({ "topics": log.topics, "data": log.data.to_vec() }),
                                block_number: log.block_number.unwrap_or_default().as_usize(),
                                tx_hash: Some(format!("{:?}", tx_hash)),
                                log_index: log.log_index.unwrap_or_default().as_usize(),
                            }).unwrap());
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
