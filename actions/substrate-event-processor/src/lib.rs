extern crate serde_json;

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Event {
    section: String,
    method: String,
    data: Vec<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    topic: String,
    brokers: Vec<String>,
    event_producer_trigger: String,
    event: String,
}

struct Action {
    params: Input,
    context: Option<Context>,
}

impl Action {
    pub fn new(params: Input) -> Self {
        Action {
            params,
            context: None,
        }
    }
    #[cfg(test)]
    pub fn init(&mut self, config: &Config) {
        let db = self.connect_db(&"http://localhost:5984".to_string(), &"test".to_string());
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db(&"http://localhost:5984".to_string(), &"test".to_string());
        self.context = Some(Context::new(db, None));
    }

    fn connect_db(&self, db_url: &str, db_name: &str) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        db
    }

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn parse_event_data(&self) -> Result<Value, Error> {
        let event: Event = serde_json::from_str(&self.params.event)?;
        return match event.section.as_str() {
            "balances" => {
                return match event.method.as_str() {
                    "Transfer" => Ok(serde_json::json!({
                        "to": event.data[1].get("AccountId").unwrap(),
                        "value": format!("{:#.4}", event.data[2].get("Balance").unwrap().parse::<f64>().unwrap() / u64::pow(10,10) as f64),
                    })),
                    _ => Ok(serde_json::json!({})),
                }
            }
            _ => Ok(serde_json::json!({})),
        };
    }

    pub fn produce_event(&mut self, event: Value) -> Result<Value, Error> {
        let event_producer_trigger = self.params.event_producer_trigger.clone();
        let topic = self.params.topic.clone();
        let brokers = self.params.brokers.clone();
        self.get_context().invoke_trigger(
            &event_producer_trigger,
            &serde_json::json!({
                "topic": topic,
                "value": event,
                "brokers": brokers
            }),
        )
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);

    #[cfg(not(test))]
    action.init();
    let parsed_event = action.parse_event_data()?;
    action.produce_event(parsed_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_data_pass() {
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "topic": "topic",
            "brokers": ["172.17.0.1:9092"],
            "event_producer_trigger": "produce_event",
            "event": {
                "section": "balances",
                "method": "Transfer",
                "meta": "[ Transfer succeeded. \\[from, to, value\\]]",
                "data": [
                    { "AccountId": "13sc83poXh93CXtzNjaCwo2Q88cS9oNyJ6Ru7DyxchqKVbbc" },
                    { "AccountId": "1N55WJHup5j1LHpbzQX6zvYu7QeLcUBd1tBp8CvA7xHGixY" },
                    { "Balance": "731000000000" }
                ]
            },
        }))
        .unwrap();
        let action = Action::new(input);

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            response,
            serde_json::json!({"to": "1N55WJHup5j1LHpbzQX6zvYu7QeLcUBd1tBp8CvA7xHGixY", "value": "73.1000"})
        );
    }
}
