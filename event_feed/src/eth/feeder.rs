use super::*;
/// The `EthFeed` struct in Rust represents a feed for Ethereum events and contracts, allowing for event
/// filtering and subscription to event feeds.
/// 
/// Properties:
/// 
/// * `eth_service`: The `eth_service` property in the `EthFeed` struct is of type `Provider<Ws>`, which
/// represents the Ethereum provider used for interacting with the Ethereum network over a WebSocket
/// connection.
/// * `events`: The `events` property in the `EthFeed` struct is a vector of tuples containing a string
/// and a `H256` value. Each tuple represents an event filter, where the string is the event name and
/// the `H256` value is the hashed representation of the event filter using the `
/// * `contracts`: The `contracts` property in the `EthFeed` struct is a vector of strings that stores
/// the contract addresses used for filtering events. It is populated based on the `contract_filter`
/// field from the `ChainConfig` struct provided during the initialization of the `EthFeed` instance.
/// The `event
pub struct EthFeed {
    eth_service: Provider<Ws>,
    events: Vec<(String, H256)>,
    contracts: Vec<String>,
}

/// This code snippet in Rust defines an implementation block for the `EthFeed` struct. Within this
/// implementation block, there are two asynchronous functions defined:
impl EthFeed {
    /// The function `new` creates a new `EthFeed` instance based on the provided `ChainConfig`
    /// configuration.
    /// 
    /// Arguments:
    /// 
    /// * `config`: The `config` parameter in the `new` function is of type `ChainConfig`. It contains
    /// the configuration settings needed to initialize an `EthFeed` instance. The `ChainConfig` struct
    /// likely includes fields such as `event_filter`, `contract_filter`, and `node_url` which are used
    /// 
    /// Returns:
    /// 
    /// The `new` function returns a `Result` containing an `EthFeed` struct.
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

    /// This Rust function asynchronously fetches event logs from a blockchain client and processes them
    /// based on specified filters and contracts, then calls a callback function with the extracted
    /// event data.
    /// 
    /// Arguments:
    /// 
    /// * `cb`: The `cb` parameter in the `event_feed` function is a callback function that takes a
    /// vector of `serde_json::Value` as input. This callback function is called with the processed
    /// event data to be handled by the caller of the `event_feed` function.
    /// 
    /// Returns:
    /// 
    /// The `event_feed` function returns a `Result<()>`, indicating that it can return either an
    /// `Ok(())` if the function completes successfully or an error if there is a problem during
    /// execution.
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
