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
    __ow_method: String,
    db_url: String,
    #[serde(default = "get_request_host")]
    endpoint: String,
    name: String,
    action_name: Option<String>,
    param_json: Option<String>,
    status:Option<String>,
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

    pub fn workflow_management(&mut self) -> Result<Value, Error> {
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
            get_namespace(),
        );
        let client = OpenwhiskClient::<NativeClient>::new(Some(&client_props));

        let param: Vec<KeyValue> = if self.params.param_json.clone().unwrap().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&self.params.param_json.clone().unwrap()).unwrap()
        };

        let trigger = Trigger {
            namespace: get_namespace(),
            name: self.params.name.clone(),

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
            name: trigger.name.clone() + "_rule",
            trigger: trigger.name.clone(),
            action: self.params.action_name.clone().unwrap(),
            status: "active".to_string(),
        };

        let rule = client
            .rules()
            .insert(&rule, true)
            .map_err(serde::de::Error::custom)?;

        {
            let doc = WorkflowDetails {
                action_name: Default::default(),
                trigger_name: trigger.name.clone(),
                rule_name: rule.name.clone(),
            };
            match self.get_context().get_document(&uuid) {
                Ok(docs) => {
                    let mut de_docs: Vec<WorkflowDetails> = serde_json::from_value(docs).unwrap();
                    de_docs.push(doc);
                    let updated_doc = serde_json::to_value(de_docs).unwrap();
                    self.get_context()
                        .update_document(&uuid, "", &updated_doc)?;
                }
                Err(_e) => {
                    let doc = serde_json::to_value(vec![doc]).unwrap();
                    self.get_context().insert_document(&doc, Some(uuid))?;
                }
            }
        }

        Ok(serde_json::json!({
            "messgae":"Rule and trigger are created",
            "trigger": trigger.name,
            "rule": rule.name}))
    }

    pub fn delete_workflow(&mut self) -> Result<Value, Error> {
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
            get_namespace(),
        );
        let client = OpenwhiskClient::<NativeClient>::new(Some(&client_props));

        client
            .triggers()
            .delete(&self.params.name)
            .map_err(serde::de::Error::custom)?;
        client
            .rules()
            .delete(&(self.params.name.clone() + "_rule"))
            .map_err(serde::de::Error::custom)?;

        Ok(serde_json::json!({"message": "Action Deleted Successfull"}))
    }

    pub fn list_workflow(&mut self) -> Result<Value, Error> {
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
            get_namespace(),
        );
        let client = OpenwhiskClient::<NativeClient>::new(Some(&client_props));

        let list = client.actions().list().map_err(serde::de::Error::custom)?;

        Ok(serde_json::json!({"Actions": list}))
    }

    pub fn workflow_rule_update(&mut self) -> Result<Value, Error> {
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
            get_namespace(),
        );
        let client = OpenwhiskClient::<NativeClient>::new(Some(&client_props));

        let rule = Rule {
            name: self.params.name.clone()+"_rule",
            trigger: "".to_string(),
            action: "".to_string(),
            status: self.params.status.clone().unwrap(),
        };

        let rule = client
            .rules()
            .insert(&rule, true)
            .map_err(serde::de::Error::custom)?;

        Ok(serde_json::json!({
            "messgae":"Action ".to_string()+ &self.params.name.clone()+"is now "+ &rule.status
        }))
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input.clone());
    #[cfg(not(test))]
    action.init();

    if input.__ow_method.clone() == "get" {
        action.list_workflow()
    } else if input.__ow_method == "put" {
        action.workflow_management()
    } else if input.__ow_method.clone() == "post" {
        action.workflow_rule_update()
    } else if input.__ow_method.clone() == "delete" {
        action.delete_workflow()
    } else {
        return Err(format!("Error in method {}", input.__ow_method.clone()))
            .map_err(serde::de::Error::custom);
    }
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
