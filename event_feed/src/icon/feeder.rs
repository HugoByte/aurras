use super::*;
use runtime::{logger::CoreLogger, Logger};

pub struct IconFeed {
    icon_service: IconService,
    events: Vec<String>,
    score: Vec<String>,
    logger: CoreLogger,
}

impl IconFeed {
    pub fn new(config: ChainConfig, logger: CoreLogger) -> Result<IconFeed> {
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
            logger,
        };

        Ok(icon_feed)
    }

    pub async fn filter(&self, transaction: &Value) -> Result<bool> {
        let mut events_filter = false;
        let mut score_filter = false;

        if !self.events.is_empty() || !self.score.is_empty() {
            let tx_hash: String =
                serde_json::from_value(transaction.get("txHash").unwrap().clone()).unwrap();
            self.logger.info(&format!(
                "Filtering the events with the tx_hash : {:?}",
                tx_hash
            ));

            let event_logs = get_event_logs_by_tx_hash(&self.icon_service, &tx_hash).await?;

            for event_log in event_logs {
                if !&self.score.is_empty() {
                    for filter_score in &self.score {
                        if filter_score == &event_log.score_address {
                            self.logger
                                .info(&format!("Matched the score filter : {:?}", filter_score));
                            score_filter = true;
                            break;
                        }
                    }
                } else {
                    self.logger
                        .info("No score filter found, allowing all the transactions");
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
                    self.logger
                        .info("No event filter found, allowing all the transactions");
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
        let result = events_filter && score_filter;
        self.logger
            .info(&format!("Filtering the result : {:?}", result));
        Ok(result)
    }

    pub async fn event_feed(&self, cb: &dyn Fn(Vec<serde_json::Value>)) -> Result<()> {
        let mut latest_height = get_icon_block_height(&self.icon_service).await?;
        let mut old_height = latest_height - 1;
        self.logger.info(&format!(
            "Event feed started at {:?}, {:?}",
            latest_height, old_height
        ));
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

                let transactions: Vec<Value> = serde_json::from_value(
                    block
                        .get("result")
                        .and_then(|val| val.get("confirmed_transaction_list"))
                        .unwrap()
                        .clone(),
                )?;

                let mut filtered_tx = Vec::<Value>::new();

                for transaction in transactions {
                    if self.filter(&transaction).await? {
                        filtered_tx.push(transaction);
                    }
                }

                self.logger.info(&format!(
                    "Filtered the {:?} transactions",
                    filtered_tx.len()
                ));

                if !filtered_tx.is_empty() {
                    cb(filtered_tx)
                }

                old_height += 1;
            } else {
                self.logger
                    .info("No new blocks got detected, sleeping for 1 second");
                sleep(Duration::from_secs(1));
            }

            latest_height = get_icon_block_height(&self.icon_service).await?;
        }
    }
}
