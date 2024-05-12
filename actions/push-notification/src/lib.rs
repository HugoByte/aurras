extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
use reqwest::StatusCode;
use types::Message;

#[cfg(test)]
use std::env;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    token: String,
    message: Message,
    api_key: String,
}

struct Action {
    params: Input,
}

impl Action {
    pub fn new(params: Input) -> Self {
        Action { params }
    }

    pub fn send_notification(&self, payload: &Value) -> Result<reqwest::blocking::Response, Error> {
        let client = reqwest::blocking::Client::new();
        client
            .post("https://fcm.googleapis.com/fcm/send")
            .bearer_auth(self.params.api_key.clone())
            .json(payload)
            .send()
            .map_err(serde::de::Error::custom)
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let action = Action::new(input);
    let response = action.send_notification(&serde_json::json!({
        "notification": {
            "title": action.params.message.title,
            "body": action.params.message.body
        },
        "to": action.params.token,
        "direct_boot_ok" : true
    }))?;
    match response.status() {
        StatusCode::OK => Ok(serde_json::json!({
            "action": "success"
        })),
        error => {
            Err(format!("failed to push notification {error:?}")).map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn send_notification_pass() {
        let action = Action::new(Input {
            // Generate Push Notification Token from client
            token: env::var("TEST_DEVICE_TOKEN").unwrap_or("".to_string()),
            message: Message {
                title: "Amount Received!".to_string(),
                body: "100 DOT".to_string(),
            },
            // Generate server token from https://console.firebase.google.com/project/<project-name>/settings/cloudmessaging
            api_key: env::var("FIREBASE_API_KEY").unwrap(),
        });

        let response = action
            .send_notification(&serde_json::json!({
                "notification": {
                    "title": action.params.message.title,
                    "body": action.params.message.body
                },
                "to": action.params.token,
                "direct_boot_ok" : true
            }))
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn send_notification_pass_main() {
        let action = json!( {
            // Generate Push Notification Token from client
            "token": env::var("TEST_DEVICE_TOKEN").unwrap_or("".to_string()),
            "message": Message {
                title: "Amount Received!".to_string(),
                body: "100 DOT".to_string(),
            },
            // Generate server token from https://console.firebase.google.com/project/<project-name>/settings/cloudmessaging
            "api_key": env::var("FIREBASE_API_KEY").unwrap(),
        });
        let response = main(action).unwrap();

        assert_eq!(response.to_string(), r#"{"action":"success"}"#);
    }

    #[test]
    #[should_panic(expected = "failed to push notification 401")]
    fn send_notification_fail_main() {
        let action = json!( {
            // Generate Push Notification Token from client
            "token": env::var("TEST_DEVICE_TOKEN").unwrap_or("".to_string()),
            "message": Message {
                title: "Amount Received!".to_string(),
                body: "100 DOT".to_string(),
            },
            // Generate server token from https://console.firebase.google.com/project/<project-name>/settings/cloudmessaging
            "api_key": "".to_string(),
        });
        main(action).unwrap();
    }
}
