use super::*;

/// The `IconFeed` struct in Rust contains an `IconService`, a vector of events, and a vector of scores.
/// 
/// Properties:
/// 
/// * `icon_service`: The `icon_service` property in the `IconFeed` struct is of type `IconService`. It
/// likely represents a service or object related to handling icons in some way.
/// * `events`: The `events` property in the `IconFeed` struct is a vector that stores strings
/// representing different events related to the icon feed. These events could include actions such as
/// "like", "comment", "share", or any other interactions that users might have with the icon feed.
/// * `score`: The `score` property in the `IconFeed` struct appears to be a vector of strings. This
/// property likely stores some kind of score information related to the events in the feed.
pub struct IconFeed {
    icon_service: IconService,
    events: Vec<String>,
    score: Vec<String>,
}

/// The `impl IconFeed` block in Rust is implementing methods for the `IconFeed` struct. Here's a
/// breakdown of what each method is doing:
impl IconFeed {
   /// The function `new` creates a new `IconFeed` instance based on the provided `ChainConfig`
   /// configuration.
   /// 
   /// Arguments:
   /// 
   /// * `config`: The `new` function you provided takes a `ChainConfig` struct as a parameter. This
   /// struct likely contains configuration details for setting up an `IconFeed`. The function then
   /// processes the `event_filter` and `contract_filter` fields from the `config` parameter to create
   /// lists of events and scores
   /// 
   /// Returns:
   /// 
   /// The `new` function is returning a `Result` containing an `IconFeed` struct.
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

    /// The function `filter` in Rust checks if a transaction meets specified event and score filters.
    /// 
    /// Arguments:
    /// 
    /// * `transaction`: The `filter` function takes a reference to a `Value` type named `transaction`
    /// as a parameter. This function filters events based on the contents of the `transaction`. The
    /// `transaction` parameter is used to extract the transaction hash (`txHash`) and then retrieve
    /// event logs associated with that transaction
    /// 
    /// Returns:
    /// 
    /// The `filter` method returns a `Result<bool>`, which indicates whether the transaction passes the
    /// filters based on the events and scores specified in the `self` object.
    pub async fn filter(&self, transaction: &Value) -> Result<bool> {
        let mut events_filter = false;
        let mut score_filter = false;

        if !self.events.is_empty() || !self.score.is_empty() {
            let tx_hash: String =
                serde_json::from_value(transaction.get("txHash").unwrap().clone()).unwrap();

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

    /// The `event_feed` function in Rust asynchronously fetches and filters transactions from a
    /// blockchain event feed.
    /// 
    /// Arguments:
    /// 
    /// * `cb`: The `cb` parameter in the `event_feed` function is a closure that takes a vector of
    /// `serde_json::Value` as input. This closure is called with the filtered transactions whenever
    /// there are new transactions that pass the filter criteria.
    /// 
    /// Returns:
    /// 
    /// The `event_feed` function returns a `Result<()>`.
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

                if !filtered_tx.is_empty() {
                    cb(filtered_tx)
                }

                old_height += 1;
            } else {
                sleep(Duration::from_secs(1));
            }

            latest_height = get_icon_block_height(&self.icon_service).await?;
        }
    }
}
