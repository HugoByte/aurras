extern crate serde_json;

mod types;

use types::user::{Claims, WorkflowDetails};

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use jsonwebtoken::{decode, DecodingKey, Validation};
use openwhisk_client_rust::*;
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
    image: String,
    file: String,
    #[serde(default)]
    parameters: Vec<KeyValue>,
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
            let db = self.connect_db(&self.params.db_url, "user_registration_db");
            let context = Context::new(db, None);
            let _data = context.get_document(&uuid)?;
        }
        let auth = self.params.openwhisk_auth.clone();
        let client_props = WskProperties::new(
            auth.to_string(),
            self.params.endpoint.clone(),
            "guest".to_string(),
        )
        .set_bypass_cerificate_check(true);
        let client = OpenwhiskClient::<NativeClient>::new(Some(&client_props));

        let action = openwhisk_client_rust::Action {
            namespace: "guest".to_string(),
            name: self.params.workflow_name.clone(),
            version: self.params.version.clone(),
            limits: Some(Limits {
                memory: Some(128),
                timeout: Some(3000),
                concurrency: Some(1),
                ..Default::default()
            }),
            exec: Exec {
                kind: self.params.kind.clone(),
                code: self.params.file.clone(),
                image: self.params.image.clone(),
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
            parameters: self.params.parameters.clone(),
        };

        let res = client.actions().insert(&action, true);
        match res {
            Ok(x) => {
                let doc = serde_json::json!({ "action_list": vec![x.clone().name] });
                match self.get_context().get_document(&uuid) {
                    Ok(docs) => {
                        let mut de_docs: WorkflowDetails = serde_json::from_value(docs).unwrap();
                        de_docs.action_list.push(x.clone().name);
                        let updated_doc = serde_json::to_value(de_docs.clone()).unwrap();
                        self.get_context()
                            .update_document(&uuid, &de_docs.rev, &updated_doc)?;
                    }
                    Err(_e) => {
                        let doc = serde_json::to_value(doc).unwrap();
                        self.get_context().insert_document(&doc, Some(uuid))?;
                    }
                }

                serde_json::to_value(x)
            }
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

fn openwhisk_auth_key() -> String {
    std::env::var("__OW_API_KEY").unwrap()
}
