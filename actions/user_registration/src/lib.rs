extern crate serde_json;

mod types;
use types::*;

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use bcrypt::{hash, DEFAULT_COST};
use chesterfield::sync::{Client, Database};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    db_name: String,
    db_url: String,
    name: String,
    email: String,
    password: String,
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
    pub fn generate_event_id(&self) -> String {
        Uuid::new_v4().to_string()
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

    pub fn register_user(&mut self) -> Result<Value, Error> {
        let hash = hash(self.params.password.clone(), DEFAULT_COST).unwrap();
        let user = User::new(self.params.name.clone(), self.params.email.clone(), hash);
        let user_id = self.generate_event_id();
        let doc = serde_json::to_value(user).unwrap();
        if let Ok(id) = self.get_context().insert_document(&doc, Some(user_id)) {
            return Ok(serde_json::json!({ "id": id }));
        }
        Err("Failed to register".to_string()).map_err(serde::de::Error::custom)
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);
    #[cfg(not(test))]
    action.init();
    action.register_user()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions_common::mock_containers::CouchDB;
    use bcrypt::verify;
    use serde_json::json;
    use tokio;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn register_user_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let input = serde_json::from_value::<Input>(json!({
            "db_name": "test",
            "db_url": url,
            "name": "test",
            "email": "test@example.com",
            "password": "testpassword",
        }))
        .unwrap();
        let mut action = Action::new(input);
        action.init(&config);
        let user_id = action.register_user().unwrap();
        let id = serde_json::from_value::<String>(user_id.get("id").unwrap().clone()).unwrap();
        let user: User =
            serde_json::from_value(action.get_context().get_document(&id).unwrap()).unwrap();

        assert_eq!(user.get_name(), &action.params.name);
        assert!(verify(action.params.password, &user.get_password()).unwrap());
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
