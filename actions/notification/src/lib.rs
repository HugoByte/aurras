extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use chesterfield::{sync::{Client, Database}};
use actions_common::{Context};
mod types;
use types::Body;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    __ow_method: String,
    __ow_body: Value,
    __ow_query: String,
    auth: String,
    db_name: String,
    db_url: String,
    event_registration_db: String,
    balance_filter_db: String
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
        let db = self.get_db(&self.params.db_url, &self.params.db_name);
        self.context = Some(Context::new(db, &self.params.auth));
    }

    #[cfg(test)]
    fn get_db(&self, db_url: &String, db_name: &String) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        db
    }

    #[cfg(not(test))]
    fn get_db(&self, db_url: &String, db_name: &String) -> Database {
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

    pub fn method(&self) -> String {
        self.params.__ow_method.clone()
    }

    pub fn get_chains(&self) -> Result<Value, Error> {
        let db = self.get_db(&self.params.db_url, &self.params.event_registration_db);
        let context = Context::new(db, &self.params.auth);       
        context.get_all()
    }
 
    pub fn get_address(&mut self, id: &String) -> Result<Value, Error> {
        self.get_context().get_document(id)
    }
    
    pub fn add_address(&self, body: &Body) -> Result<String, Error> {
        let db = self.get_db(&self.params.db_url, &format!("{}_{}", &body.topic, &self.params.balance_filter_db));
        let mut context = Context::new(db, &self.params.auth);
        context.insert_document(serde_json::json!({
            "token": body.token
        }), Some(body.address.clone())).map_err(serde::de::Error::custom)   
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args).unwrap();
    let mut action = Action::new(input);
    action.init();

    match action.method().as_ref() {
        "POST" => {
            let body = serde_json::from_value::<Body>(action.params.__ow_body.clone()).unwrap();
            let id = action.add_address(&body)?;
            return action.get_address(&id)
        },
        "GET" => return action.get_chains(),
        method => return Err(format!("method not supported document {}", method)).map_err(serde::de::Error::custom)
    }
}