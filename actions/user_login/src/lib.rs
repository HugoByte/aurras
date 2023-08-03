extern crate serde_json;

mod types;

use chrono::{Duration, Utc};
use types::{user::Claims, *};

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use bcrypt::verify;
use chesterfield::sync::{Client, Database};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    db_name: String,
    db_url: String,
    email: String,
    password: String,
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

    pub fn login_user(&mut self) -> Result<Value, Error> {
        let id = self.params.email.clone();
        let user_id_res = self.get_context().get_document(&id)?;
        let user_id: UserId = serde_json::from_value(user_id_res).unwrap();
        let res = self.get_context().get_document(&user_id.user_id)?;
        let user: User = serde_json::from_value(res).unwrap();
        if verify(self.params.password.clone(), &user.get_password()).unwrap() {
            let headers = Header::default();
            let encoding_key =
                EncodingKey::from_secret("user_registration_token_secret_key".as_bytes());
            let now = Utc::now() + Duration::days(1); // Expires in 1 day
            let claims = Claims {
                sub: user_id.user_id,
                exp: now.timestamp(),
            };
            let user_token = encode(&headers, &claims, &encoding_key).unwrap();
            Ok(serde_json::json!({ "user_token": user_token }))
        } else {
            Err("Enter Valid password".to_string()).map_err(serde::de::Error::custom)
        }
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);
    #[cfg(not(test))]
    action.init();
    action.login_user()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use bcrypt::{hash, DEFAULT_COST};
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use serde_json::json;
    use tokio;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn user_login_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let input = serde_json::from_value::<Input>(json!({
            "db_name": "test",
            "db_url": url,
            "email":"test@example.com",
            "password": "testpassword",

        }))
        .unwrap();
        let mut action = Action::new(input);
        action.init(&config);
        let hash = hash("testpassword", DEFAULT_COST).unwrap();
        let user = json!({
            "name": "test",
            "email": "test@example.com",
            "password": hash,
        });
        let id = uuid::Uuid::new_v4().to_string();
        let doc_id = serde_json::to_value(UserId::new(id.clone())).unwrap();
        let _id_store= action
            .get_context()
            .insert_document(&doc_id, Some("test@example.com".to_string()))
            .unwrap();
        let user_id = action
            .get_context()
            .insert_document(&user, Some(id.clone()));
        assert_eq!(user_id.unwrap(), id);
        let user_token = action.login_user().unwrap();
        let token: String =
            serde_json::from_value::<String>(user_token.get("user_token").unwrap().clone())
                .unwrap();
        let decoding_key =
            DecodingKey::from_secret("user_registration_token_secret_key".as_bytes());
        let validation = Validation::default();
        let data = decode::<Claims>(&token, &decoding_key, &validation);

        assert_eq!(data.unwrap().claims.sub, id);

        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test]
    #[should_panic(expected = "error fetching document")]
    async fn user_login_fail() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let input = serde_json::from_value::<Input>(json!({
            "db_name": "test",
            "db_url": url,
            "email":"131",
            "password": "password",

        }))
        .unwrap();
        let mut action = Action::new(input);
        action.init(&config);
        let hash = hash("testpassword", DEFAULT_COST).unwrap();
        let user = json!({
            "name": "test",
            "email": "test@example.com",
            "password": hash,
        });
        let id = uuid::Uuid::new_v4().to_string();
        let user_id = action
            .get_context()
            .insert_document(&user, Some(id.clone()));
        assert_eq!(user_id.unwrap(), id);
        action.login_user().unwrap();

        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
