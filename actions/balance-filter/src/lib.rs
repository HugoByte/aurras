extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
use actions_common::Context;

#[cfg(test)]
use actions_common::Config;

use chesterfield::sync::{Client, Database};
use types::{Message, Payload};

#[cfg(test)]
use types::{Address, Deposit, Topic};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    messages: Vec<Message>,
    push_notification_trigger: String,
    db_name: String,
    db_url: String,
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
        let db = self.connect_db(&self.params.db_url, &self.params.db_name);
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db(&self.params.db_url, &self.params.db_name);
        self.context = Some(Context::new(db, None));
    }

    fn connect_db(&self, db_url: &str, db_name: &str) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        if !db.exists().unwrap() {
            db.create().unwrap();
        }
        db
    }

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    fn filter_topics(&mut self) -> Payload {
        let mut payload = vec![];
        for message in self.params.messages.clone().iter() {
            if let Ok(topic) = self.get_context().get_document(&message.topic) {
                message.parse_value().unwrap();
                if message.parse_value().is_ok() {
                    payload.push((
                        message.parse_value().unwrap(),
                        serde_json::from_value(topic).unwrap(),
                    ))
                }
            }
        }
        payload
    }

    fn filter_address(&self, payload: Payload) -> Payload {
        payload
            .into_iter()
            .filter(|message| message.1.filters.contains_key(&message.0.address))
            .collect()
    }

    pub fn invoke_trigger(&mut self, payload: Payload) -> Result<Value, Error> {
        let mut failed_triggers = vec![];
        for message in payload.iter() {
            let trigger = self.params.push_notification_trigger.clone();
            // TODO: Add attributes neccessary for push notification trigger
            if self
                .get_context()
                .invoke_trigger(
                    &trigger,
                    &serde_json::json!({
                        "token": message.1.filters.get(&message.0.address).unwrap().token,
                        "message": {
                            "title": "Amount Recieved!",
                            "body": message.0.amount
                        }
                    }),
                )
                .is_err()
            {
                failed_triggers.push(message.0.address.clone());
            }
        }
        if !failed_triggers.is_empty() {
            return Err(format!("error in triggers {}", failed_triggers.join(", ")))
                .map_err(serde::de::Error::custom);
        }
        Ok(serde_json::json!({
            "action": "success"
        }))
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);

    // TODO: Fix
    #[cfg(not(test))]
    action.init();

    let filtered_topics = action.filter_topics();
    let filtered_address = action.filter_address(filtered_topics);
    action.invoke_trigger(filtered_address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use tokio;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn filter_topics_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let topic = "1234".to_string();
        let messages = vec![
            Message {
                topic: "mytopic".to_string(),
                value: serde_json::json!({
                    "from": "12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf",
                    "to":"15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw",
                    "value": "100.0000".to_string(),
                })
                .to_string(),
            },
            Message {
                topic: "1234".to_string(),
                value: serde_json::json!({
                    "from": "12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf",
                    "to":"15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw",
                    "value": "100.0000".to_string(),
                })
                .to_string(),
            },
        ];
        let mut action = Action::new(Input {
            push_notification_trigger: "push_notification".to_string(),
            db_url: url,
            db_name: "test".to_string(),
            messages,
        });
        action.init(&config);

        action
            .get_context()
            .insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.clone()),
            )
            .unwrap();

        let mut doc: Topic =
            serde_json::from_value(action.get_context().get_document(&topic).unwrap()).unwrap();

        doc.filters.insert(
            "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            Address {
                token: "1".to_string(),
            },
        );

        action
            .get_context()
            .update_document(
                &topic,
                &doc.rev,
                &serde_json::to_value(doc.clone()).unwrap(),
            )
            .unwrap();
        let doc: Topic =
            serde_json::from_value(action.get_context().get_document(&topic).unwrap()).unwrap();
        let expected = vec![(
            Deposit {
                address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
                amount: "100.0000".to_string(),
            },
            Topic {
                id: doc.id,
                rev: doc.rev,
                filters: doc.filters,
            },
        )];
        assert_eq!(action.filter_topics(), expected);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    async fn filter_address_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let topic = "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string();
        let _messages = vec![
            Message {
                topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
                value: serde_json::json!({
                    "from": "12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf",
                    "to":"12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf",
                    "value": "100.0000".to_string(),
                })
                .to_string(),
            },
            Message {
                topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
                value: serde_json::json!({
                    "from": "12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf",
                    "to":"15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw",
                    "value": "100.0000".to_string(),
                })
                .to_string(),
            },
        ];

        let input = serde_json::from_value::<Input>(serde_json::json!({
            "push_notification_trigger": "push_notification",
            "db_name": "test",
            "db_url": url,
            "messages": [{
                "topic":"418a8b8c-02b8-11ec-9a03-0242ac130003",
                "value": "{\"from\":\"12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf\",\"to\":\"15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw\",\"value\":\"100.0000\"}"
            }]
        })).unwrap();

        let mut action = Action::new(input);
        action.init(&config);

        action
            .get_context()
            .insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.clone()),
            )
            .unwrap();

        let mut doc: Topic =
            serde_json::from_value(action.get_context().get_document(&topic).unwrap()).unwrap();

        doc.filters.insert(
            "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            Address {
                token: "1".to_string(),
            },
        );

        action
            .get_context()
            .update_document(
                &topic,
                &doc.rev,
                &serde_json::to_value(doc.clone()).unwrap(),
            )
            .unwrap();
        let doc: Topic =
            serde_json::from_value(action.get_context().get_document(&topic).unwrap()).unwrap();
        let expected = vec![(
            Deposit {
                address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
                amount: "100.0000".to_string(),
            },
            Topic {
                id: doc.id,
                rev: doc.rev,
                filters: doc.filters,
            },
        )];
        let filtered_topics = action.filter_topics();
        assert_eq!(action.filter_address(filtered_topics), expected);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    // TODO: This panic because of reqwest blocking in tokio runtime context. Should Add sync or async context.
    #[ignore]
    #[should_panic]
    #[tokio::test(flavor = "multi_thread")]
    async fn invoke_trigger_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let topic = "1234".to_string();
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "push_notification_trigger": "test",
            "db_name": "test",
            "db_url": url,
            "messages": [{
                "topic":"418a8b8c-02b8-11ec-9a03-0242ac130003",
                "value": "{\"from\":\"12o3hWM94g5EoNkEiPibo7WMToM6gKvL8osJCGht9W79iEpf\",\"to\":\"15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw\",\"value\":1000}"
            }]
        })).unwrap();
        
        let mut action = Action::new(input);
        action.init(&config);

        action
            .get_context()
            .insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.clone()),
            )
            .unwrap();

        let mut doc: Topic =
            serde_json::from_value(action.get_context().get_document(&topic).unwrap()).unwrap();

        doc.filters.insert(
            "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            Address {
                token: "1".to_string(),
            },
        );

        action
            .get_context()
            .update_document(
                &topic,
                &doc.rev,
                &serde_json::to_value(doc.clone()).unwrap(),
            )
            .unwrap();
        let filtered_topics = action.filter_topics();
        assert_eq!(
            action
                .invoke_trigger(action.filter_address(filtered_topics))
                .unwrap(),
            serde_json::json!({})
        );
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
