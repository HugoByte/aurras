extern crate serde_json;
mod types;

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use types::*;

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
        return match self.params.event.event.as_str() {
            "CallMessage" => {
                return Ok(serde_json::json!({
                    "data": serde_json::from_value::<String>(self.params.event.args[4].clone()).unwrap(),
                    "req_id": i64::from_str_radix(&self.params.event.topics[3].clone()[2..], 16).unwrap(),
                    "to": self.params.event.topics[2].clone(),
                    "from": self.params.event.topics[1].clone()
                }));
            }
            _ => Err(serde::de::Error::custom(format!(
                "Section {} Not Defined",
                self.params.event.event
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
                "blockNumber": 365,
                "blockHash": "0xa6918392f79acd0983ad8b42ba59daaf07589d6204cc68defc057cda84ac8e32",
                "transactionIndex": 0,
                "removed": false,
                "address": "0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82",
                "data": "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001d73656e6443616c6c4d6573736167655f69636f6e305f68617264686174000000",
                "topics": [
                    "0x2cbc78425621c181f9f8a25fc06e44a0ac2b67cd6a31f8ed7918934187f8cc59",
                    "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c",
                    "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911",
                    "0x0000000000000000000000000000000000000000000000000000000000000001"
                ],
                "transactionHash": "0x3cf2006eaa5520175facf358e3a8d50c6d0de20067f957152726542543647132",
                "logIndex": 0,
                "event": "CallMessage",
                "eventSignature": "CallMessage(string,string,uint256,uint256,bytes)",
                "args": [
                    {
                    "_isIndexed": true,
                    "hash": "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c"
                    },
                    {
                    "_isIndexed": true,
                    "hash": "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"
                    },
                    { "type": "BigNumber", "hex": "0x01" },
                    { "type": "BigNumber", "hex": "0x01" },
                    "0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174"
                ]
                },
        }))
        .unwrap();
        let action = Action::new(input);

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            response,
            serde_json::json!({"data":"0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174","from":"0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c","req_id":1,"to":"0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"}
            )
        );
    }

    #[test]
    fn parse_call_sent_event_data_pass() {
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "topic": "topic",
            "brokers": ["172.17.0.1:9092"],
            "event_producer_trigger": "produce_staking_event",
            "event": {
                "blockNumber": 365,
                "blockHash": "0xa6918392f79acd0983ad8b42ba59daaf07589d6204cc68defc057cda84ac8e32",
                "transactionIndex": 0,
                "removed": false,
                "address": "0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82",
                "data": "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001d73656e6443616c6c4d6573736167655f69636f6e305f68617264686174000000",
                "topics": [
                    "0x2cbc78425621c181f9f8a25fc06e44a0ac2b67cd6a31f8ed7918934187f8cc59",
                    "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c",
                    "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911",
                    "0x0000000000000000000000000000000000000000000000000000000000000001"
                ],
                "transactionHash": "0x3cf2006eaa5520175facf358e3a8d50c6d0de20067f957152726542543647132",
                "logIndex": 0,
                "event": "CallMessage",
                "eventSignature": "CallMessage(string,string,uint256,uint256,bytes)",
                "args": [
                    {
                    "_isIndexed": true,
                    "hash": "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c"
                    },
                    {
                    "_isIndexed": true,
                    "hash": "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"
                    },
                    { "type": "BigNumber", "hex": "0x01" },
                    { "type": "BigNumber", "hex": "0x01" },
                    "0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174"
                ]
                },
        }))
        .unwrap();
        let action = Action::new(input);

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            serde_json::from_value::<String>(response["from"].clone()).unwrap(),
            "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c".to_string()
        );
    }

    #[test]
    #[should_panic(expected = "Section CallMessageSent Not Defined")]
    fn parse_call_message_sent_event_data_method_exception() {
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "topic": "topic",
            "brokers": ["172.17.0.1:9092"],
            "event_producer_trigger": "produce_staking_event",
            "event": {
                "blockNumber": 365,
                "blockHash": "0xa6918392f79acd0983ad8b42ba59daaf07589d6204cc68defc057cda84ac8e32",
                "transactionIndex": 0,
                "removed": false,
                "address": "0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82",
                "data": "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001d73656e6443616c6c4d6573736167655f69636f6e305f68617264686174000000",
                "topics": [
                    "0x2cbc78425621c181f9f8a25fc06e44a0ac2b67cd6a31f8ed7918934187f8cc59",
                    "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c",
                    "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911",
                    "0x0000000000000000000000000000000000000000000000000000000000000001"
                ],
                "transactionHash": "0x3cf2006eaa5520175facf358e3a8d50c6d0de20067f957152726542543647132",
                "logIndex": 0,
                "event": "CallMessageSent",
                "eventSignature": "CallMessageSent(string,string,uint256,uint256,bytes)",
                "args": [
                    {
                    "_isIndexed": true,
                    "hash": "0xff4555761731a3642885c2ca67fb09e827825033bdf74f1c7f9b4b3151eec14c"
                    },
                    {
                    "_isIndexed": true,
                    "hash": "0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"
                    },
                    { "type": "BigNumber", "hex": "0x01" },
                    { "type": "BigNumber", "hex": "0x01" },
                    "0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174"
                ]
                },
        }))
        .unwrap();
        let action = Action::new(input);

        action.parse_event_data().unwrap();
    }
}
