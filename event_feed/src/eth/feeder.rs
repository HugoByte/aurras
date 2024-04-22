use super::*;
pub struct EthFeed {
    eth_service: Provider<Ws>,
    events: Vec<String>,
    contracts: Vec<String>,
}

impl EthFeed {
    pub async fn new(config: ChainConfig) -> Result<EthFeed> {
        let events = config
            .event_filter
            .split_inclusive("),")
            .map(|evt| evt.trim_end_matches(',').to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let contracts = config
            .contract_filter
            .split(",")
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

    pub async fn event_feed(&self, cb: &dyn Fn( Vec<serde_json::Value>)) -> Result<()> {
        let client = Arc::new(&self.eth_service);

        let last_block = client
            .get_block(BlockNumber::Latest)
            .await?
            .unwrap()
            .number
            .unwrap();

        let filter = Filter::new()
            .from_block(last_block - 25)
            .events(self.events.clone());

        let mut stream = client.subscribe_logs(&filter).await?;

        while let Some(log) = stream.next().await {
            if self.contracts.is_empty() || self.contracts.contains(&format!("{:?}", &log.address))
            {
                let tx_hash = match log.transaction_hash {
                    Some(hash) => hash,
                    None => {
                        let data = json!({"receipt": {"logs": [log]} });
                        cb(vec![data]);
                        continue;
                    }
                };

                let tx_receipt = client.get_transaction_receipt(tx_hash).await;

                if let Ok(Some(receipt)) = tx_receipt {
                    let data = json!({"logIndex": log.log_index, "receipt": receipt});
                    cb(vec![data]);
                }
            }
        }

        Ok(())
    }
}
