use super::*;

#[derive(Clone, Default)]
pub struct Filter {
    pub events: Option<Vec<String>>,
    pub score: Option<Vec<String>>,
}

impl Filter {
    pub async fn filter(&self, icon_service: &IconService, transaction: &Value) -> Result<bool> {
        let mut events_filter = false;
        let mut score_filter = false;

        if self.events.is_some() || self.score.is_some() {
            let tx_hash: String =
                serde_json::from_value(transaction.get("txHash").unwrap().clone()).unwrap();

            let event_logs = get_event_logs_by_tx_hash(icon_service, &tx_hash).await?;

            for event_log in event_logs {
                if let Some(filter_score_list) = &self.score {
                    for filter_score in filter_score_list {
                        if filter_score == &event_log.score_address {
                            score_filter = true;
                            break;
                        }
                    }
                } else {
                    score_filter = true;
                }

                if let Some(filter_events_list) = &self.events {
                    for filter_event in filter_events_list {
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
