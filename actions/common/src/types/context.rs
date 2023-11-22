use chesterfield::sync::Database;
use reqwest::StatusCode;

use super::Trigger;
use serde_json::{to_value, Error, Value};
use std::env;

#[cfg(test)]
use tokio::runtime::Handle;

#[derive(new, Debug, Clone)]
pub struct Config {
    #[new(value = r#""test:test".to_string()"#)]
    pub api_key: String,
    #[new(value = r#""http://172.17.0.1:8888".to_string()"#)]
    pub host: String,
    #[new(value = r#""action".to_string()"#)]
    pub name: String,
    #[new(value = r#""guest".to_string()"#)]
    pub namespace: String,
}

pub struct Context {
    pub host: String,
    pub name: String,
    pub namespace: String,
    db: Database,
    user: String,
    pass: String,
}

#[cfg(not(test))]
fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .unwrap()
}

#[cfg(test)]
fn client() -> reqwest::Client {
    reqwest::Client::new()
}

#[cfg(test)]
fn invoke_client(request: reqwest::RequestBuilder) -> Result<reqwest::Response, reqwest::Error> {
    let handle = Handle::current();
    tokio::task::block_in_place(move || handle.block_on(async { request.send().await }))
}

#[cfg(not(test))]
fn invoke_client(
    request: reqwest::blocking::RequestBuilder,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    request.send()
}

impl Context {
    pub fn new(db: Database, config: Option<&Config>) -> Self {
        let api_key = if env::var("__OW_API_KEY").is_ok() {
            env::var("__OW_API_KEY").unwrap()
        } else {
            match config {
                Some(config) => config.api_key.clone(),
                None => "test:test".to_string(),
            }
        };
        let auth: Vec<&str> = api_key.split(':').collect();
        let host = if env::var("__OW_API_HOST").is_ok() {
            env::var("__OW_API_HOST").unwrap()
        } else {
            match config {
                Some(config) => config.host.clone(),
                None => "host.docker.internal".to_string(),
            }
        };
        let name = if env::var("__OW_ACTION_NAME").is_ok() {
            env::var("__OW_ACTION_NAME").unwrap()
        } else {
            match config {
                Some(config) => config.name.clone(),
                None => "action".to_string(),
            }
        };
        let namespace = if env::var("__OW_NAMESPACE").is_ok() {
            env::var("__OW_NAMESPACE").unwrap()
        } else {
            match config {
                Some(config) => config.namespace.clone(),
                None => "guest".to_string(),
            }
        };
        Context {
            host,
            db,
            name,
            namespace,
            user: auth[0].to_string(),
            pass: auth[1].to_string(),
        }
    }

    pub fn invoke_action(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client();
        let url = format!(
            "{}/api/v1/namespaces/{}/actions/{}",
            self.host, self.namespace, name
        );
        let response = invoke_client(
            client
                .post(url)
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        )
        .map_err(serde::de::Error::custom)?;
        match response.status().is_success() {
            true => Ok(serde_json::json!({
                "success": true
            })),
            false => Err(format!(
                "failed to invoke action {} {:?}",
                name,
                response.error_for_status()
            ))
            .map_err(serde::de::Error::custom),
        }
    }

    pub fn invoke_trigger(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client();
        let url = format!(
            "{}/api/v1/namespaces/{}/triggers/{}?result=true",
            self.host, self.namespace, name
        );
        let response = invoke_client(
            client
                .post(url)
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        )
        .map_err(serde::de::Error::custom)?;
        match response.status().is_success() {
            true => Ok(serde_json::json!({
                "success": true
            })),
            false => Err(format!(
                "failed to invoke trigger {} {:?}",
                name,
                response.error_for_status()
            ))
            .map_err(serde::de::Error::custom),
        }
    }

    // TODO: Fix return
    pub fn create_rule(&self, name: &str, trigger: &str, action: &str) -> Result<Value, Error> {
        let client = client();
        let url = format!(
            "{}/api/v1/namespaces/{}/rules/{}?overwrite=true",
            self.host, self.namespace, name
        );
        let response = invoke_client(
            client
                .put(url)
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(&serde_json::json!({
                    "status": "",
                    "action": format!("/{}/{}",self.namespace, action),
                    "trigger": format!("/{}/{}",self.namespace, trigger)
                })),
        )
        .map_err(serde::de::Error::custom)?;
        match response.status().is_success() {
            true => Ok(serde_json::json!({
                "success": true
            })),
            false => Err(format!(
                "failed to create rule {} {:?}",
                name,
                response.error_for_status()
            ))
            .map_err(serde::de::Error::custom),
        }
    }

    // TODO: Fix return
    pub fn create_trigger(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client();
        let url = format!(
            "{}/api/v1/namespaces/{}/triggers/{}?overwrite=true",
            self.host, self.namespace, name
        );
        let response = invoke_client(
            client
                .put(url.clone())
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        )
        .map_err(serde::de::Error::custom)?;
        match response.status().is_success() {
            true => to_value(Trigger::new(name.to_string(), url)),
            false => Err(format!(
                "failed to create trigger {} {:?}",
                name,
                response.error_for_status()
            ))
            .map_err(serde::de::Error::custom),
        }
    }

    pub fn update_document(&self, id: &str, rev: &str, doc: &Value) -> Result<String, Error> {
        match self.db.update(doc, id, rev).send() {
            Ok(r) => Ok(r.id),
            Err(err) => Err(format!("error updating document {doc}: {err:?}"))
                .map_err(serde::de::Error::custom),
        }
    }

    pub fn get_auth_key(&self) -> String {
        format!("{}:{}", self.user, self.pass)
    }

    pub fn insert_document(&self, doc: &Value, id: Option<String>) -> Result<String, Error> {
        match self.db.insert(doc, id).send() {
            Ok(r) => Ok(r.id),
            Err(err) => Err(format!("error creating document {doc}: {err:?}"))
                .map_err(serde::de::Error::custom),
        }
    }

    pub fn get_document(&self, id: &str) -> Result<Value, Error> {
        match self.db.get(id).send::<Value>() {
            Ok(v) => Ok(v.into_inner().unwrap()),
            Err(err) => Err(format!("error fetching document {id}: {err:?}"))
                .map_err(serde::de::Error::custom),
        }
    }

    pub fn get_list(&self, db_url: &str, db_name: &str) -> Result<Value, Error> {
        let client = client();
        let url = format!("{db_url}/{db_name}/_all_docs?include_docs=true");
        if let Ok(response) = invoke_client(client.get(url)) {
            return match response.status() {
                StatusCode::OK => {
                    #[cfg(not(test))]
                    return response.json().map_err(serde::de::Error::custom);

                    #[cfg(test)]
                    {
                        let handle = Handle::current();
                        return tokio::task::block_in_place(move || {
                            handle.block_on(async { response.json().await })
                        })
                        .map_err(serde::de::Error::custom);
                    }
                }
                _ => {
                    Err(format!("error fetching list {db_name}")).map_err(serde::de::Error::custom)
                }
            };
        };

        Err(format!("error fetching list {db_name}")).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[cfg(feature = "mock_containers")]
mod tests {
    use super::*;
    use crate::mock::mock_containers::CouchDB;
    use chesterfield::sync::Client;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::HashMap;
    use tokio;
    use tokio::time::{sleep, Duration};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Topic {
        #[serde(skip_serializing, rename(deserialize = "_id"))]
        pub id: String,
        #[serde(skip_serializing, rename(deserialize = "_rev"))]
        pub rev: String,
        pub filters: HashMap<String, Address>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Address {
        pub token: String,
        pub trigger: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Row<T> {
        rows: Vec<View<T>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct View<T> {
        doc: T,
    }

    fn connect_db(db_url: &String, db_name: &String) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        if !db.exists().unwrap() {
            db.create().unwrap();
        }
        db
    }

    #[tokio::test]
    async fn update_document_pass() {
        let topic = "1234".to_string();
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        context
            .insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.clone()),
            )
            .unwrap();

        let mut doc: Topic = serde_json::from_value(context.get_document(&topic).unwrap()).unwrap();

        doc.filters.insert(
            topic.clone(),
            Address {
                token: "1".to_string(),
                trigger: "1".to_string(),
            },
        );
        context
            .update_document(
                &topic,
                &doc.rev,
                &serde_json::to_value(doc.clone()).unwrap(),
            )
            .unwrap();

        let doc: Topic = serde_json::from_value(context.get_document(&topic).unwrap()).unwrap();
        let mut expected = HashMap::new();
        expected.insert(
            topic.clone(),
            Address {
                token: "1".to_string(),
                trigger: "1".to_string(),
            },
        );
        assert_eq!(doc.filters, expected);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_list_pass() {
        let config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        let topic = "1234".to_string();

        context
            .insert_document(
                &serde_json::json!({
                    "filters": {}
                }),
                Some(topic.clone()),
            )
            .unwrap();

        let mut doc: Topic = serde_json::from_value(context.get_document(&topic).unwrap()).unwrap();

        doc.filters.insert(
            topic.clone(),
            Address {
                token: "1".to_string(),
                trigger: "1".to_string(),
            },
        );
        context
            .update_document(
                &topic,
                &doc.rev,
                &serde_json::to_value(doc.clone()).unwrap(),
            )
            .unwrap();
        let doc: Topic = serde_json::from_value(context.get_document(&topic).unwrap()).unwrap();
        let docs: Row<Topic> =
            serde_json::from_value(context.get_list(&url, &"test".to_string()).unwrap()).unwrap();
        let expected: View<Topic> = View {
            doc: Topic { ..doc },
        };
        assert_eq!(docs.rows, vec![expected]);
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn create_trigger_pass() {
        let topic = "1234".to_string();
        let mut config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(format!(
                "/api/v1/namespaces/{}/triggers/{}",
                config.namespace, topic
            )))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        config.host = mock_server.uri();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        context
            .create_trigger(&topic, &serde_json::json!({}))
            .unwrap();
        let received_requests = mock_server.received_requests().await;
        assert!(received_requests.is_some());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn create_rule_pass() {
        let topic = "1234".to_string();
        let mut config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(format!(
                "/api/v1/namespaces/{}/rules/{}",
                config.namespace, topic
            )))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        config.host = mock_server.uri();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        context.create_rule(&topic, "trigger", "action").unwrap();
        let received_requests = mock_server.received_requests().await;
        assert!(received_requests.is_some());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn invoke_trigger_pass() {
        let mut config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v1/namespaces/{}/triggers/{}",
                config.namespace, "trigger"
            )))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        config.host = mock_server.uri();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        context
            .invoke_trigger("trigger", &serde_json::json!({}))
            .unwrap();
        let received_requests = mock_server.received_requests().await;
        assert!(received_requests.is_some());
        couchdb.delete().await.expect("Stopping Container Failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn invoke_action_pass() {
        let mut config = Config::new();
        let couchdb = CouchDB::new("admin".to_string(), "password".to_string())
            .await
            .unwrap();
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v1/namespaces/{}/actions/{}",
                config.namespace, "action"
            )))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        config.host = mock_server.uri();
        sleep(Duration::from_millis(5000)).await;
        let url = format!("http://admin:password@localhost:{}", couchdb.port());
        let context = Context::new(connect_db(&url, &"test".to_string()), Some(&config));
        context
            .invoke_action("action", &serde_json::json!({}))
            .unwrap();
        let received_requests = mock_server.received_requests().await;
        assert!(received_requests.is_some());
        couchdb.delete().await.expect("Stopping Container Failed");
    }
}
