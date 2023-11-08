extern crate serde_json;
use actions_common::Context;
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;

use jsonwebtoken::{decode, DecodingKey, Validation};

use types::Claims;
use types::{Response, Topic, UserData};

#[cfg(test)]
use actions_common::Config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct Input {
    __ow_method: String,
    #[serde(default = "empty_string")]
    address: String,
    workflow_management_db: String,
    db_name: String,
    db_url: String,
    event_registration_db: String,
    #[serde(default = "empty_string")]
    auth_token: String,
    #[serde(default = "empty_string")]
    topic: String,
    #[serde(default = "empty_string")]
    token: String,
    #[serde(default)]
    input: Value,
    #[serde(default = "default_status")]
    status: String,
}

fn empty_string() -> String {
    String::new()
}

fn default_status() -> String {
    String::from("active")
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

    #[allow(dead_code)]
    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn method(&self) -> String {
        self.params.__ow_method.clone()
    }

    pub fn get_event_sources(&self) -> Result<Value, Error> {
        let db = self.connect_db(&self.params.db_url, &self.params.event_registration_db);
        let context = Context::new(db, None);
        let list: Response = serde_json::from_value(
            context.get_list(&self.params.db_url, &self.params.event_registration_db)?,
        )?;
        Ok(serde_json::json!({
            "statusCode": 200,
            "headers": { "Content-Type": "application/json" },
            "body": list.rows
        }))
    }

    pub fn user_validate(&self) -> Result<String, Error> {
        let decoding_key =
            DecodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let validation = Validation::default();
        let claim = decode::<Claims>(&self.params.auth_token, &decoding_key, &validation).unwrap();
        let uuid = claim.claims.sub;
        {
            let db = self.connect_db(&self.params.db_url, &"user_registration_db".to_string());
            let context = Context::new(db, None);
            let _data = context.get_document(&uuid)?;
            Ok(uuid)
        }
    }

    pub fn add_data_to_db(&mut self) -> Result<String, Error> {
        let user_id = self.user_validate()?;

        let mut db_input = self.params.input.clone();
        db_input["token"] = serde_json::json!(self.params.token.clone());
        let topic = self.params.topic.clone();

        let db = self.connect_db(&self.params.db_url, &self.params.workflow_management_db);
        let context = Context::new(db, None);
        if context.get_document(&topic).is_err() {
            context.insert_document(
                &serde_json::json!({
                    "data": [{
                        "user_id": user_id,
                        "status": String::from("active"),
                        "input_data": db_input
                    }]
                }),
                Some(topic.to_string()),
            )
        } else {
            let mut doc: Topic = serde_json::from_value(context.get_document(&topic)?)?;
            let mut user_index = None;
            for (index, user) in doc.data.iter().enumerate() {
                if user.user_id == user_id {
                    user_index = Some(index);
                }
            }
            match user_index {
                Some(x) => doc.data[x].input_data = db_input,
                None => {
                    let new_user = UserData {
                        user_id,
                        status: "active".to_string(),
                        input_data: db_input,
                    };
                    doc.data.push(new_user);
                }
            }

            context.update_document(&topic, &doc.rev, &serde_json::to_value(doc.clone())?)
        }
    }

    pub fn update_subscription_status(&mut self) -> Result<String, Error> {
        let user_id = self.user_validate()?;
        let db = self.connect_db(&self.params.db_url, &self.params.workflow_management_db);
        let context = Context::new(db, None);
        let topic = self.params.topic.clone();

        let mut doc: Topic = serde_json::from_value(context.get_document(&topic)?)?;
        let mut user_index = None;
        for (index, user) in doc.data.iter().enumerate() {
            if user.user_id == user_id {
                user_index = Some(index);
            }
        }
        match user_index {
            Some(x) => doc.data[x].status = self.params.status.clone(),
            None => (),
        }

        context.update_document(&topic, &doc.rev, &serde_json::to_value(doc.clone())?)
    }
    pub fn delete_subscription_status(&mut self) -> Result<String, Error> {
        let user_id = self.user_validate()?;
        let db = self.connect_db(&self.params.db_url, &self.params.workflow_management_db);
        let context = Context::new(db, None);
        let topic = self.params.topic.clone();

        let mut doc: Topic = serde_json::from_value(context.get_document(&topic)?)?;
        let mut user_index = None;
        for (index, user) in doc.data.iter().enumerate() {
            if user.user_id == user_id {
                user_index = Some(index);
            }
        }

        match user_index {
            Some(x) => {
                doc.data.remove(x);
            }
            None => {
                return Err(format!("User didnt subscribed this service",))
                    .map_err(serde::de::Error::custom)
            }
        }

        context.update_document(&topic, &doc.rev, &serde_json::to_value(doc.clone())?)
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    #[allow(unused_mut)]
    let mut action = Action::new(input);

    // TODO: Fix
    #[cfg(not(test))]
    action.init();

    match action.method().as_ref() {
        "post" => {
            let _id = action.add_data_to_db()?;
            Ok(serde_json::json!({
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": {
                    "success": true
                }
            }))
        }
        "get" => action.get_event_sources(),
        "put" => {
            action.update_subscription_status()?;
            Ok(serde_json::json!({
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": {
                    "success": true
                }
            }))
        }
        "delete" => {
            action.delete_subscription_status()?;
            Ok(serde_json::json!({
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": {
                    "success": true
                }
            }))
        }

        method => Err(format!("method not supported document {}", method))
            .map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use actions_common::Config;
    use chrono::{Duration as ChronoDuration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use tokio;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        name: String,
        email: String,
        password: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct UserId {
        pub user_id: String,
    }

    #[tokio::test]
    async fn filter_topics_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();

        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            ..Default::default()
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        let event_registration_db =
            action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context
            .insert_document(
                &serde_json::json!({
                    "name": "polkadot",
                    "trigger": "trigger"
                }),
                Some("event_id".to_string()),
            )
            .unwrap();
        let workflow_db =
            action.connect_db(&action.params.db_url, &action.params.workflow_management_db);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let _res = action.add_data_to_db();
        let _res = action.add_data_to_db();
        let res_data =
            workflow_management_db_context.get_document("418a8b8c-02b8-11ec-9a03-0242ac130003");
        let res = serde_json::from_value::<Topic>(res_data.unwrap());
        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    async fn add_data_to_db() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());

        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            ..Default::default()
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        let event_registration_db =
            action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context
            .insert_document(
                &serde_json::json!({
                    "name": "polkadot",
                    "trigger": "trigger"
                }),
                Some("event_id".to_string()),
            )
            .unwrap();
        let workflow_db =
            action.connect_db(&action.params.db_url, &action.params.workflow_management_db);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let _res = action.add_data_to_db();
        let _res = action.add_data_to_db();
        let res_data =
            workflow_management_db_context.get_document("418a8b8c-02b8-11ec-9a03-0242ac130003");
        let res = serde_json::from_value::<Topic>(res_data.unwrap());
        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    async fn add_two_user_data_to_db() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());

        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();

        let _topic = "1234".to_string();
        let _address = "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string();
        let _token = "1".to_string();
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            ..Default::default()
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        let event_registration_db =
            action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context
            .insert_document(
                &serde_json::json!({
                    "name": "polkadot",
                    "trigger": "trigger"
                }),
                Some("event_id".to_string()),
            )
            .unwrap();
        let workflow_db =
            action.connect_db(&action.params.db_url, &action.params.workflow_management_db);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let _res = action.add_data_to_db();
        let _res = action.add_data_to_db();

        let user = User {
            name: "test".to_string(),
            email: "test1@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            ..Default::default()
        });
        action.init(&config);

        user_db_context
            .insert_document(&uder_id_doc, Some("test1@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();
        let _res = action.add_data_to_db();

        let res_data =
            workflow_management_db_context.get_document("418a8b8c-02b8-11ec-9a03-0242ac130003");
        let res = serde_json::from_value::<Topic>(res_data.unwrap());
        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    async fn update_subscription_status() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();

        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            status: "inactive".to_string(),
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        let event_registration_db =
            action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context
            .insert_document(
                &serde_json::json!({
                    "name": "polkadot",
                    "trigger": "trigger"
                }),
                Some("event_id".to_string()),
            )
            .unwrap();
        let workflow_db =
            action.connect_db(&action.params.db_url, &action.params.workflow_management_db);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let _res = action.add_data_to_db();
        let _res = action.update_subscription_status();

        let res_data =
            workflow_management_db_context.get_document("418a8b8c-02b8-11ec-9a03-0242ac130003");
        let res = serde_json::from_value::<Topic>(res_data.unwrap());
        println!("{:?}", res);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    async fn remove_workflow_subscription() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();

        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            status: "active".to_string(),
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        let event_registration_db =
            action.connect_db(&action.params.db_url, &action.params.event_registration_db);
        let event_registration_db_context = Context::new(event_registration_db, None);

        event_registration_db_context
            .insert_document(
                &serde_json::json!({
                    "name": "polkadot",
                    "trigger": "trigger"
                }),
                Some("event_id".to_string()),
            )
            .unwrap();
        let workflow_db =
            action.connect_db(&action.params.db_url, &action.params.workflow_management_db);
        let workflow_management_db_context = Context::new(workflow_db, None);
        let _res = action.add_data_to_db();
        let _res = action.delete_subscription_status();

        let res_data =
            workflow_management_db_context.get_document("418a8b8c-02b8-11ec-9a03-0242ac130003");
        let res = serde_json::from_value::<Topic>(res_data.unwrap());
        assert!(res.is_ok());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    #[should_panic(expected = "User didnt subscribed this service")]
    async fn remove_subscription_fails() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let user = User {
            name: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };
        let user_id = Uuid::new_v4().to_string();
        let doc = serde_json::to_value(user.clone()).unwrap();
        let uder_id_doc = serde_json::to_value(UserId {
            user_id: user_id.clone(),
        })
        .unwrap();

        let headers = Header::default();
        let encoding_key =
            EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let now = Utc::now() + ChronoDuration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: user_id.clone(),
            exp: now.timestamp(),
        };
        let user_token = encode(&headers, &claims, &encoding_key).unwrap();

        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let mut action = Action::new(Input {
            db_url: url.clone(),
            db_name: "test".to_string(),
            __ow_method: "post".to_string(),
            address: "15ss3TDX2NLG31ugk6QN5zHhq2MUfiaPhePSjWwht6Dr9RUw".to_string(),
            workflow_management_db: "workflow_management_db".to_string(),
            event_registration_db: "event_registration_db".to_string(),
            auth_token: user_token,
            topic: "418a8b8c-02b8-11ec-9a03-0242ac130003".to_string(),
            token: "akjDSIJGFIJHNSdmngknomlmxcgknhNDlnglnlkoNSDG".to_string(),
            input: serde_json::json!({
                "url": "".to_string(),
                "owner_key": "".to_string(),
                "validator": "".to_string(),
            }),
            status: "active".to_string(),
        });
        action.init(&config);

        let user_db = action.connect_db(&action.params.db_url, &"user_registration_db".to_string());
        let user_db_context = Context::new(user_db, None);
        user_db_context
            .insert_document(&uder_id_doc, Some("test@example.com".to_string()))
            .unwrap();
        user_db_context
            .insert_document(&doc, Some(user_id.clone()))
            .unwrap();

        action.add_data_to_db().unwrap();
        action.delete_subscription_status().unwrap();
        let res = action.delete_subscription_status();
        couchdb.delete().await.expect("Stopping Container Failed");
        res.unwrap();
    }
}
