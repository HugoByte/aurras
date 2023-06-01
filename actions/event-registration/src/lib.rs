extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use uuid::Uuid;
mod types;
#[cfg(test)]
use actions_common::Config;
use actions_common::{Context, Trigger};
use chesterfield::sync::{Client, Database};
use types::Source;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    name: String,
    db_name: String,
    db_url: String,
    feed: String,
    brokers: Vec<String>
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

    pub fn generate_event_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn register_source(&mut self, topic: &str, trigger: &str) -> Result<Value, Error> {
        let source = Source::new(self.params.name.to_string(), trigger.to_string());
        let doc = serde_json::to_value(source).unwrap();
        if let Ok(id) = self
            .get_context()
            .insert_document(&doc, Some(topic.to_string()))
        {
            let doc = self.get_context().get_document(&id)?;
            return serde_json::from_value(doc);
        }
        Err("Failed to register".to_string()).map_err(serde::de::Error::custom)
    }

    pub fn register_trigger(&mut self, topic: &str) -> Result<Value, Error> {
        let feed = self.params.feed.clone();
        let namespace = self.get_context().namespace.clone();
        let auth_key = self.get_context().get_auth_key();
        let brokers = self.params.brokers.clone();
        let trigger = self.get_context().create_trigger(
            topic,
            &serde_json::json!({
                "annotations": [{
                    "key": "feed",
                    "value": format!("/{}/{}", namespace, feed)
                }],
                "parameters": [{
                    "key": "topic",
                    "value": topic
                }]
            }),
        );
        self.get_context().invoke_action(
            &feed,
            &serde_json::json!({
                "triggerName": format!("/{}/{}", namespace, topic),
                "lifecycleEvent": "CREATE",
                "authKey": auth_key,
                "topic": topic,
                "brokers": brokers
            }),
        )?;
        trigger
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);
    let event_id = action.generate_event_id();
    #[cfg(not(test))]
    action.init();
    let trigger = serde_json::from_value::<Trigger>(action.register_trigger(&event_id)?)?;
    action.register_source(&event_id, &trigger.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use serde_json::json;
    use tokio;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn register_source_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let input = serde_json::from_value::<Input>(json!({
            "feed": "kafka-provider-feed",
            "name": "polkadot",
            "db_name": "test",
            "db_url": url,
            "brokers": ["172.17.0.1:9092"]

        }))
        .unwrap();
        let mut action = Action::new(input);
        let event_id = action.generate_event_id();
        action.init(&config);

        action
            .register_source(&event_id, &"trigger".to_string())
            .unwrap();
        let source: Source =
            serde_json::from_value(action.get_context().get_document(&event_id).unwrap()).unwrap();

        assert_eq!(
            source,
            Source::new(action.params.name.clone(), "trigger".to_string())
        );
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
