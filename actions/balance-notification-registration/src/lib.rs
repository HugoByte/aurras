extern crate serde_json;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
use std::collections::HashMap;
use types::{Address, Topic};

#[cfg(test)]
use actions_common::Config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    __ow_method: String,
    #[serde(default = "empty_string")]
    __ow_query: String,
    #[serde(default = "empty_string")]
    address: String,
    balance_filter_db: String,
    db_name: String,
    db_url: String,
    event_registration_db: String,
    #[serde(default = "empty_string")]
    token: String,
    #[serde(default = "empty_string")]
    topic: String,
}

fn empty_string() -> String {
    String::new()
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

    fn connect_db(&self, db_url: &String, db_name: &String) -> Database {
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

    pub fn method(&self) -> String {
        self.params.__ow_method.clone()
    }

    pub fn get_event_sources(&self) -> Result<Value, Error> {
        let db = self.connect_db(&self.params.db_url, &self.params.event_registration_db);
        let context = Context::new(db, None);
        context.get_list(&self.params.db_url, &self.params.event_registration_db)
    }

    pub fn get_address(&mut self, id: &String) -> Result<Value, Error> {
        self.get_context().get_document(id)
    }

    pub fn add_address(&self, token: &str, topic: &str, address: &str) -> Result<String, Error> {
        let db = self.connect_db(&self.params.db_url, &self.params.balance_filter_db);
        let context = Context::new(db, None);
        if context.get_document(topic).is_err() {
            context.insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.to_string()),
            )?;
        }
        let mut doc: Topic = serde_json::from_value(context.get_document(topic)?)?;

        doc.filters.insert(
            address.to_string(),
            Address {
                token: token.to_string(),
            },
        );
        context.update_document(&topic, &doc.rev, &serde_json::to_value(doc.clone())?)
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args).unwrap();
    let mut action = Action::new(input);

    // TODO: Fix
    #[cfg(not(test))]
    action.init();

    match action.method().as_ref() {
        "post" => {
            let id = action.add_address(
                &action.params.token,
                &action.params.topic,
                &action.params.address,
            )?;
            return action.get_address(&id);
        }
        "get" => return action.get_event_sources(),
        method => {
            return Err(format!("method not supported document {}", method))
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use tokio;
    use tokio::time::{sleep, Duration};
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Source {
        name: String,
        trigger: String
    }
    impl Source {
        pub fn new(name: String, trigger: String) -> Self {
            Source { name, trigger }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Row<T> {
        rows: Vec<View<T>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct View<T> {
        doc: T,
    }
    
    #[tokio::test]
    async fn add_address_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let topic = "1234".to_string();
        let address = "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string();
        let token = "1".to_string();
        let mut action = Action::new(Input {
            db_url: url,
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            __ow_query: "".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            balance_filter_db: "balance_filter_db".to_string(),
            event_registration_db: "".to_string(),
            token: "1".to_string(),
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
        });
        action.init(&config);

        let db = action.connect_db(&action.params.db_url, &action.params.balance_filter_db);
        let context = Context::new(db, None);

        action.add_address(&token, &topic, &address).unwrap();
        let doc: Topic =
            serde_json::from_value(context.get_document(&topic).unwrap()).unwrap();
        let mut filters = HashMap::new();
        filters.insert(
            address.clone(),
            Address {
                token: "1".to_string(),
            },
        );
        let expected = Topic {
            id: doc.id.clone(),
            rev: doc.rev.clone(),
            filters,
        };
        assert_eq!(doc, expected);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    // TODO: This panic because of reqwest blocking in tokio runtime context. Should Add sync or async context.
    #[should_panic]
    #[tokio::test]
    async fn get_event_sources_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let topic = "1234".to_string();
        let address = "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string();
        let token = "1".to_string();
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            __ow_query: "".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            balance_filter_db: "balance_filter_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            token: "1".to_string(),
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
        });
        action.init(&config);

        let event_registration_db = action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context.insert_document(&serde_json::json!({
            "name": "polkadot",
            "trigger": "trigger"
        }), Some("event_id".to_string())).unwrap();
        let doc: Source = serde_json::from_value(event_registration_db_context.get_document(&"event_id".to_string()).unwrap()).unwrap();
        let sources: Row<Source> =
            serde_json::from_value(event_registration_db_context.get_list(&url.clone(), &action.params.event_registration_db).unwrap()).unwrap();
        let expected: View<Source> = View {
            doc: Source { ..doc },
        };
        assert_eq!(sources.rows, vec![expected]);
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
