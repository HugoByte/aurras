extern crate serde_json;

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Event {
    #[serde(rename = "scoreAddress")]
    score_address: String,
    indexed: Vec<String>,
    data: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    topic: String,
    brokers: Vec<String>,
    event_producer_trigger: String,
    event: Event,
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
    #[allow(dead_code)]
    #[cfg(test)]
    pub fn init(&mut self, config: &Config) {
        let db = self.connect_db("http://localhost:5984", "test");
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db("http://localhost:5984", "test");
        self.context = Some(Context::new(db, None));
    }

    #[allow(dead_code)]
    fn connect_db(&self, db_url: &str, db_name: &str) -> Database {
        let client = Client::new(db_url).unwrap();
        client.database(db_name).unwrap()
    }

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn parse_event_data(&self) -> Result<Value, Error> {
        return match self.params.event.indexed[0].as_str() {
            "CallMessage(str,str,int,int,bytes)" => {
                return Ok(serde_json::json!({
                    "data": self.params.event.data[1],
                    // "req_id": self.params.event.data[0],
                    "req_id":  i64::from_str_radix(&self.params.event.data[0].clone()[2..], 16).unwrap(),
                    "to": self.params.event.indexed[2],
                    "from": self.params.event.indexed[1]
                }));
            }
            _ => Err(serde::de::Error::custom(format!(
                "Section {} Not Defined",
                self.params.event.indexed[0]
            ))),
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
            "event":  {
                    "scoreAddress": "cxca002001f46a19562648d3348bc9080175dcad3a",
                    "indexed": [
                        "CallMessage(str,str,int,int,bytes)",
                        "btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1",
                        "cxdf97f11ef352727f5d826eec68e569d6471aa9a4",
                        "0x1"
                    ],
                    "data": [
                        "0x1",
                        "0x73656e6443616c6c4d6573736167655f686172646861745f69636f6e30"
                    ]
                },
        }))
        .unwrap();
        let action = Action::new(input);

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            response,
            serde_json::json!({"data":"0x73656e6443616c6c4d6573736167655f686172646861745f69636f6e30","to":"cxdf97f11ef352727f5d826eec68e569d6471aa9a4","req_id":1,"from":"btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1"})
        );
    }

    #[test]
    fn parse_call_sent_event_data_pass() {
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "topic": "topic",
            "brokers": ["172.17.0.1:9092"],
            "event_producer_trigger": "produce_staking_event",
            "event": {
                "scoreAddress": "cxca002001f46a19562648d3348bc9080175dcad3a",
                "indexed": [
                    "CallMessage(str,str,int,int,bytes)",
                    "btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1",
                    "cxdf97f11ef352727f5d826eec68e569d6471aa9a4",
                    "0x1"
                ],
                "data": [
                    "0x1",
                    "0x73656e6443616c6c4d6573736167655f686172646861745f69636f6e30"
                ]
            },
        }))
        .unwrap();
        let action = Action::new(input);

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            serde_json::from_value::<String>(response["from"].clone()).unwrap(),
            "btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1".to_string()
        );
    }

    #[test]
    #[should_panic(expected = "Section CallMessageSent(str,str,int,int,bytes) Not Defined")]
    fn parse_call_message_sent_event_data_method_exception() {
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "topic": "topic",
            "brokers": ["172.17.0.1:9092"],
            "event_producer_trigger": "produce_staking_event",
            "event": {
                "scoreAddress": "cxca002001f46a19562648d3348bc9080175dcad3a",
                "indexed": [
                    "CallMessageSent(str,str,int,int,bytes)",
                    "btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1",
                    "cxdf97f11ef352727f5d826eec68e569d6471aa9a4",
                    "0x1"
                ],
                "data": [
                    "0x1",
                    "0x73656e6443616c6c4d6573736167655f686172646861745f69636f6e30"
                ]
            },
        }))
        .unwrap();
        let action = Action::new(input);

        action.parse_event_data().unwrap();
    }
}
