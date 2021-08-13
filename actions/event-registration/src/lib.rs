extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use uuid::Uuid;
mod types;
use chesterfield::{sync::{Client, Database}};
use types::{Source};
use actions_common::{Context, Trigger};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    name: String,
    auth: String,
    db_name: String,
    db_url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Output {
    topic: String,
}

struct Action {
    params: Input,
    context: Option<Context>
}

impl Action {

    pub fn new(params: Input) -> Self {
        Action { params, context: None }
    }

    pub fn init(&mut self) {
        let db = self.connect_db(&self.params.db_url, &self.params.db_name);
        self.context = Some(Context::new(db, &self.params.auth));
    }

    #[cfg(test)]
    fn connect_db(&self, db_url: &String, db_name: &String) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        db
    }

    #[cfg(not(test))]
    fn connect_db(&self, db_url: &String, db_name: &String) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        if !db.exists().unwrap() {
            db.create().unwrap();
        }
        db
    }

    pub fn get_context(&mut self) -> &mut Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn generate_event_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn register_source(&mut self, topic: &str, trigger: &str) -> Result<Value, Error> {
        let source = Source::new(self.params.name.to_string(), topic.to_string(), trigger.to_string());
        let doc = serde_json::to_value(source).unwrap();
        if let Ok(id) = self.get_context().insert_document(doc, None) {
            let doc = self.get_context().get_document(&id)?;
            return serde_json::from_value(doc)
        }
        Err("Failed to register".to_string()).map_err(serde::de::Error::custom)
    }

    pub fn register_trigger(&mut self, topic: &str) -> Result<Value, Error> {
        self.get_context().create_trigger(topic)
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args).unwrap();
    let mut action = Action::new(input);
    let event_id = action.generate_event_id();
    action.init();
    let trigger = serde_json::from_value::<Trigger>(action.register_trigger(&event_id)?)?;
    action.register_source(&event_id, &trigger.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn generate_event_id() {
        let input = serde_json::from_value::<Input>(json!({
            "name": "node-template",
            "auth": "789c46b1-71f6-4ed5-8c54-816aa4f8c502:abczO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
            "db_name": "test",
            "db_url": "http://localhost:5984"

        })).unwrap();
        let mut action = Action::new(input);
        let event_id = action.generate_event_id();
        action.init();
    }
}