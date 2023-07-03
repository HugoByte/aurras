extern crate serde_json;

mod types;

use types::user::{Claims, WorkflowDetails};

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use jsonwebtoken::{decode, DecodingKey, Validation};
use openwhisk_rust::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    _ow_method: String,
    db_url: String,
    #[serde(default = "get_request_host")]
    endpoint: String,
    operation: String,
    trigger_name: String,
    action_name: String,
    param_json: String,
    auth_token: String,
    #[serde(default = "openwhisk_auth_key")]
    openwhisk_auth: String,
}

struct Action {
    pub params: Input,
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
        let db = self.connect_db(&self.params.db_url, "workflow_registration");
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db(&self.params.db_url, "workflow_registration");
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

    pub fn workflow_registration(&mut self) -> Result<Value, Error> {
        let decoding_key =
            DecodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let validation = Validation::default();
        let claim = decode::<Claims>(&self.params.auth_token, &decoding_key, &validation).unwrap();
        let uuid = claim.claims.sub;
        {
            let db = self.connect_db(&self.params.db_url, "user_registration");
            let context = Context::new(db, None);
            let _data = context.get_document(&uuid)?;
        }
        let auth = self.params.openwhisk_auth.clone();
        let client_props = WskProperties::new(
            auth.to_string(),
            self.params.endpoint.clone(),
            true,
            "guest".to_string(),
        );
        let client = OpenwhiskClient::<WasmClient>::new(Some(&client_props));

        let param: Vec<KeyValue> = if self.params.param_json.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&self.params.param_json).unwrap()
        };

        let trigger = Trigger {
            namespace: get_namespace(),
            name: self.params.trigger_name.clone(),

            annotations: vec![KeyValue {
                key: "feed".to_string(),
                value: serde_json::json!("feed"),
            }],
            parameters: param,

            version: "0.0.1".to_string(),
            ..Default::default()
        };
        let trigger = client
            .triggers()
            .insert(&trigger, true)
            .map_err(serde::de::Error::custom)?;

        let rule = Rule {
            name: trigger.name.clone() + "_" + &self.params.action_name,
            trigger: trigger.name.clone(),
            action: self.params.action_name.clone(),
        };

        let rule = client
            .rules()
            .insert(&rule, true)
            .map_err(serde::de::Error::custom)?;

        Ok(serde_json::json!({
            "messgae":"Rule and trigger are created",
            "trigger": trigger.name,
            "rule": rule.name}))
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);
    #[cfg(not(test))]
    action.init();
    action.workflow_registration()
}

fn get_request_host() -> String {
    std::env::var("__OW_API_HOST").unwrap()
}
fn get_namespace() -> String {
    std::env::var("__OW_NAMESPACE").unwrap_or("guest".to_string())
}

fn openwhisk_auth_key() -> String {
    "AAAAkbdB3Jk:APA91bGYmzmAJ6Vq6u-qHNK3Sf7OnMKWJSZy5LJYeGSnJ9hSeBz7K8Indv7t-jEbXGDM2waQ519wkISI6pUN7845zO9gOwjnQRXZ0wHMaVfV4ziGtBIhdfVwfSOMGSR0F_d8pmdFiuXq".to_string()
}
