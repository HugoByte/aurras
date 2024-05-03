use super::*;

pub struct IconFeed {
    icon_service: IconService,
    events: Vec<String>,
    score: Vec<String>,
}

impl IconFeed {
    pub fn new(config: ChainConfig) -> Result<IconFeed> {
        let events = config
            .event_filter
            .split(',')
            .map(|evt| evt.to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let score = config
            .contract_filter
            .split(',')
            .map(|evt| evt.to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let icon_service = icon_service::IconService::new(Some(config.node_url));

        let icon_feed = IconFeed {
            icon_service,
            events,
            score,
        };

        Ok(icon_feed)
    }

    pub async fn filter(&self, transaction: &Value) -> Result<bool> {
        let mut events_filter = false;
        let mut score_filter = false;

        if !self.events.is_empty() || !self.score.is_empty() {
            // let tx_hash: String =
            //     serde_json::from_value(transaction.get("txHash").unwrap().clone()).unwrap();
            let tx_hash: String = serde_json::from_value(
                transaction.get("txHash")
                    .ok_or_else(|| anyhow::anyhow!("Transaction hash not found in transaction"))?
                    .clone()
            )?;

            let event_logs = get_event_logs_by_tx_hash(&self.icon_service, &tx_hash).await?;

            for event_log in event_logs {
                if !&self.score.is_empty() {
                    for filter_score in &self.score {
                        if filter_score == &event_log.score_address {
                            score_filter = true;
                            break;
                        }
                    }
                } else {
                    score_filter = true;
                }

                if !&self.events.is_empty() {
                    for filter_event in &self.events {
                        if event_log.indexed_events[0].starts_with(&format!("{filter_event}(")) {
                            events_filter = true;
                            break;
                        }
                    }
                } else {
                    events_filter = true;
                }

                if events_filter && score_filter {
                    events_filter = true;
                    break;
                }
            }
        } else {
            events_filter = true;
            score_filter = true;
        }

        Ok(events_filter & score_filter)
    }

    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        let mut latest_height = get_icon_block_height(&self.icon_service).await?;
        let mut old_height = latest_height - 1;

        loop {
            if old_height < latest_height {
                let block = match self
                    .icon_service
                    .get_block_by_height(&format!("0x{:x}", latest_height))
                    .await
                {
                    Ok(block) => block,
                    Err(_) => {
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                };

                let transactions = match block.get("result")
                .and_then(|val| val.get("confirmed_transaction_list"))
                .cloned() {
                    Some(transactions) => transactions,
                    None => {
                        eprintln!("No transactions found in block {}", latest_height);
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                };

            let transactions: Vec<Value> = serde_json::from_value(transactions)?;

                let mut filtered_tx = Vec::<Value>::new();

                for transaction in transactions {
                    if self.filter(&transaction).await? {
                        filtered_tx.push(transaction);
                    }
                }

                if !filtered_tx.is_empty() {
                    cb(filtered_tx)
                }

                old_height += 1;
            } else {
                sleep(Duration::from_secs(1));
            }

            latest_height = match get_icon_block_height(&self.icon_service).await {
                Ok(height) => height,
                Err(err) => {
                    eprintln!("Error getting latest block height: {}", err);
                    continue;
                }
            }
        }
    }
}
