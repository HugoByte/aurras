extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
use crate::types::update_with;
#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use types::Message;
use types::Topic;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    messages: Vec<Message>,
    event_registration_db: String,
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

    pub fn fetch_input(&mut self) -> Result<Vec<Value>, Error> {
        let id = self.params.messages.clone()[0].topic.clone();
        let data = self.get_context().get_document(&id)?;
        let parsed = serde_json::from_value::<Topic>(data)?;
        Ok(parsed.data)
    }

    // fetching the action name from database
    fn get_action_name(&mut self) -> Result<String, Error> {

        // creating connection with the event registration database
        let db = self.connect_db(&self.params.db_url, &self.params.event_registration_db);
        let context = Context::new(db, None);

        let data:Value= match context.get_document(&self.params.messages[0].topic){
            Ok(document) => document,
            Err(_)  => {
                return Err(format!("topic {} is not exists in the database", &self.params.messages[0].topic))
                .map_err(serde::de::Error::custom);
            }
        };

        // getting action name from the document
        let action_name:String = serde_json::from_value(data.get("name").unwrap().clone()).unwrap();
        Ok(action_name)
    }

    pub fn invoke_action(&mut self, payload: &mut Vec<Value>) -> Result<Value, Error> {

        let mut failed_actions = vec![];

        let action_name = self.get_action_name().unwrap();

        for message in payload.iter_mut() {
            let data = serde_json::from_str::<Value>(&self.params.messages[0].value).unwrap();
            update_with(message, &data);
            
            if self
                .get_context()
                .invoke_action(&action_name, &serde_json::json!({"data": message}))
                .is_err()
            {
                failed_actions.push(self.params.messages[0].value.clone()); 
            }
        }
        if !failed_actions.is_empty() {
            return Err(format!("error in triggers {:?}", failed_actions))
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

    let mut payload = action.fetch_input()?;
    action.invoke_action(&mut payload)
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
            event_registration_db: "event_registration_db".to_string(),
            messages: vec![Message {
                topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
                value: serde_json::json!({ "era" :0}).to_string(),
            }],
        });
        action.init(&config);
        let workflow_db = action.connect_db(&action.params.db_url, &action.params.db_name);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let doc = serde_json::json!({
            "data": [{ "url": "todo!()".to_string(), "validator": "todo!()".to_string(), "owner_key": "todo!()".to_string() }]
        });
        let _ = workflow_management_db_context
            .insert_document(&doc, Some(action.params.messages[0].topic.clone()));
        let res = action.fetch_input();

        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[test]
    fn test_update_value() {
        let action = Action::new(Input {
            db_url: "url".to_string(),
            db_name: "test".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            messages: vec![Message {
                topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
                value: serde_json::json!({ "era" :0}).to_string(),
            }],
        });

        let mut doc = serde_json::json!({
            "url": "todo!()".to_string(), "validator": "todo!()".to_string(), "owner_key": "todo!()".to_string() }
        );
        let data = serde_json::from_str::<Value>(&action.params.messages[0].value).unwrap();
        update_with(&mut doc, &data);
        assert_eq!(
            doc,
            serde_json::json!({"url":"todo!()","era":0,"owner_key":"todo!()","validator":"todo!()"})
        )
    }

    #[tokio::test]
    async fn get_action_name_pass(){
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            messages: vec![Message {
                topic: "0a36fd24-84ac-420e-9187-912929c782ea".to_string(),
                value: serde_json::json!({ "era" :0}).to_string(),
            }],
        });
        action.init(&config);

        let event_registration_db = action.connect_db(&action.params.db_url, "event_registration_db");
        let event_registration_db_context = Context::new(event_registration_db, None);

        let json_value = r#"{"name": "icon-eth-notification", "trigger": "0a36fd24-84ac-420e-9187-912929c782ea"}"#;
        let doc:Value = serde_json::from_str(json_value).unwrap();

        let _ = event_registration_db_context
            .insert_document(&doc, Some(action.params.messages[0].topic.clone()));

        let invoke_action_name = action.get_action_name().unwrap();
        assert_eq!(&invoke_action_name, "icon-eth-notification");

        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    #[should_panic="topic 0a36fd24-84ac-420e-9187-912929c782ea is not exists in the database"]
    async fn get_action_name_fail(){
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            messages: vec![Message {
                topic: "0a36fd24-84ac-420e-9187-912929c782ea".to_string(),
                value: serde_json::json!({ "era" :0}).to_string(),
            }],
        });
        action.init(&config);

        let invoke_action_name = action.get_action_name().unwrap();
        assert_eq!(&invoke_action_name, "icon-eth-notification");

        couchdb.delete().await.expect("Stopping Container Failed");
    }
}