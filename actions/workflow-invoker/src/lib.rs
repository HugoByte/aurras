extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
use actions_common::Context;

#[cfg(test)]
use actions_common::Config;

use chesterfield::sync::{Client, Database};
use types::Message;

use types::DbDatas;
// #[cfg(test)]
use types::{Era, Topic};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    messages: Vec<Message>,
    polkadot_payout_trigger: String,
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

    pub fn fetch_input(&mut self) -> Result<Vec<DbDatas>, Error> {
        let id = self.params.messages.clone()[0].topic.clone();
        let data = self.get_context().get_document(&id)?;
        println!("{:?}", data);
        let parsed = serde_json::from_value::<Topic>(data)?;
        Ok(parsed.data)
    }

    pub fn invoke_trigger(&mut self, payload: Vec<DbDatas>) -> Result<Value, Error> {
        let mut failed_triggers = vec![];
        for message in payload.iter() {
            let era = serde_json::from_str::<Era>(&self.params.messages[0].value)?;
            let trigger = self.params.polkadot_payout_trigger.clone();
            if self
                .get_context()
                .invoke_trigger(
                    &trigger,
                    &serde_json::json!({"allowed_hosts": [message.endpoint.clone()],
                    "data": {
                        "address": message.validator,
                        "era": era.era,
                        "owner_key": message.key,
                        "url": message.endpoint
                    }}),
                )
                .is_err()
            {
                failed_triggers.push(message.validator.clone());
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

    let payload = action.fetch_input()?;
    println!("22   {:?}", payload);
    action.invoke_trigger(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use actions_common::Config;
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
        let _topic = "1234".to_string();
        let _address = "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string();
        let _token = "1".to_string();
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            polkadot_payout_trigger: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            messages: vec![Message {
                topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
                value: "{ \"era\" :0}".to_string(),
            }],
        });
        action.init(&config);
        let workflow_db = action.connect_db(&action.params.db_url, &action.params.db_name);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let doc = serde_json::json!({
            "data": [DbDatas{ endpoint: "todo!()".to_string(), validator: "todo!()".to_string(), key: "todo!()".to_string() }]
        });
        workflow_management_db_context
            .insert_document(&doc, Some(action.params.messages[0].topic.clone()));
        let res = action.fetch_input();
        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
