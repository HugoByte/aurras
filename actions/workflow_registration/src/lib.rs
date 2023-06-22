extern crate serde_json;

mod types;

use chrono::{Duration, Utc};
use types::{user::Claims, *};

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use bcrypt::verify;
use chesterfield::sync::{Client, Database};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use openwhisk_rust::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    db_url: String,
    #[serde(default = "get_request_host")]
    endpoint: String,
    workflow_name: String,
    version: String,
    kind: String,
    file: String,
    auth_token: String,
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
        let auth = "AAAAkbdB3Jk:APA91bGYmzmAJ6Vq6u-qHNK3Sf7OnMKWJSZy5LJYeGSnJ9hSeBz7K8Indv7t-jEbXGDM2waQ519wkISI6pUN7845zO9gOwjnQRXZ0wHMaVfV4ziGtBIhdfVwfSOMGSR0F_d8pmdFiuXq";
        let client_props = WskProperties::new(
            auth.to_string(),
            self.params.endpoint.clone(),
            true,
            "guest".to_string(),
        );
        let client = OpenwhiskClient::<WasmClient>::new(Some(&client_props));

        let mut image = String::new();
        if self.params.kind == "rust:1.34".to_string() {
            image = "openwhisk/action-rust-v1.34".to_string()
        } else {
            image = "hugobyte/openwhisk-runtime-rust:v0.3".to_string()
        }

        let action = openwhisk_rust::Action {
            namespace: "guest".to_string(),
            name: self.params.workflow_name.clone(),
            version: self.params.version.clone(),
            limits: Default::default(),
            exec: Exec {
                kind: self.params.kind.clone(),
                code: self.params.file.clone(),
                image,
                init: "".to_string(),
                main: "".to_string(),
                components: vec![],
                binary: true,
            },
            error: "".to_string(),
            publish: true,
            updated: 0,
            annotations: vec![KeyValue {
                key: "feed".to_string(),
                value: serde_json::json!({}),
            }],
        };

        let res = client.actions().insert(&action, true);
        match res {
            Ok(x) => serde_json::to_value(x),
            Err(e) => return Err(e).map_err(serde::de::Error::custom),
        }
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
