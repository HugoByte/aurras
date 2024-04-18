use super::*;

#[derive(Clone, Default, Debug)]
pub struct Filter {
    pub events: Vec<String>,
    pub score: Vec<String>,
}

impl Filter {
    pub async fn filter(&self, icon_service: &IconService, transaction: &Value) -> Result<bool> {
        let mut events_filter = false;
        let mut score_filter = false;

        if !self.events.is_empty() || !self.score.is_empty() {
            let tx_hash: String =
                serde_json::from_value(transaction.get("txHash").unwrap().clone()).unwrap();

            let event_logs = get_event_logs_by_tx_hash(icon_service, &tx_hash).await?;

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
}
