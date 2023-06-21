use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
mod types;
#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
use chesterfield::sync::{Client, Database};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    brokers: Vec<String>,
    event: String,
    topic: String,
    #[serde(rename = "eventProcessor")]
    event_processor: String,
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
    #[allow(dead_code)]
    #[cfg(test)]
    pub fn init(&mut self, config: &Config) {
        let db = self.connect_db("http://localhost:5984", "test");
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db("http://localhost:5984", "test");
        self.context = Some(Context::new(db, None));
    }

    #[allow(dead_code)]
    fn connect_db(&self, db_url: &str, db_name: &str) -> Database {
        let client = Client::new(db_url).unwrap();
        client.database(db_name).unwrap()
    }

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn parse_event_data(&self) -> Result<Value, Error> {
        let event = self.params.event.clone();
        let brokers = self.params.brokers.clone();
        let topic = self.params.topic.clone();
        Ok(serde_json::json!({
            "event": serde_json::from_str::<serde_json::Value>(&event)?,
            "brokers": brokers,
            "topic": topic
        }))
    }

    pub fn process_event(&mut self, value: &Value) -> Result<Value, Error> {
        let event_processor = self.params.event_processor.clone();
        self.get_context().invoke_action(
            &event_processor,
            value,
        )
    }
}
pub fn main(args: Value) -> Result<Value, Error> {
    // TODO: Use processor for each event source to process event to generic format as the event receiver will be generic for all event source
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);

    #[cfg(not(test))]
    action.init();
    action.process_event(&action.parse_event_data()?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_event_pass() {
        let action = Action::new(Input {
            brokers: vec!["172.17.0.1:9092".to_string()],
            event: "{\"section\": \"balances\", \"method\": \"Transfer\", \"data\": [{\"AccountId\":\"148fP7zCq1JErXCy92PkNam4KZNcroG9zbbiPwMB1qehgeT4\"},{\"AccountId\":\"13bbv2rNzAKuT2oSkFJyHUJAmPVbBYNQbRQ95xW3sQBGffHa\"},{\"Balance\":\"24682100255\"}]}".to_string(),
            topic: "7231ea34-7bc2-44e8-8601-c8cceb78f8c3".to_string(),
            event_processor: "substrate_event_processor".to_string()
        });

        let response = action.parse_event_data().unwrap();

        assert_eq!(
            response,
            serde_json::json!({
                "event": {"section": "balances", "method": "Transfer", "data": [{"AccountId":"148fP7zCq1JErXCy92PkNam4KZNcroG9zbbiPwMB1qehgeT4"},{"AccountId":"13bbv2rNzAKuT2oSkFJyHUJAmPVbBYNQbRQ95xW3sQBGffHa"},{"Balance":"24682100255"}]},
                "topic": "7231ea34-7bc2-44e8-8601-c8cceb78f8c3",
                "brokers": ["172.17.0.1:9092"],
            })
        );
    }

    #[test]
    #[should_panic(expected = "Action not Initialized!")]
    fn parse_event_fail() {
        let action = json!( {
            "brokers": vec!["172.17.0.1:9092".to_string()],
            "event": "{\"section\": \"balances\", \"method\": \"Transfer\", \"data\": [{\"AccountId\":\"148fP7zCq1JErXCy92PkNam4KZNcroG9zbbiPwMB1qehgeT4\"},{\"AccountId\":\"13bbv2rNzAKuT2oSkFJyHUJAmPVbBYNQbRQ95xW3sQBGffHa\"},{\"Balance\":\"24682100255\"}]}".to_string(),
            "topic": "7231ea34-7bc2-44e8-8601-c8cceb78f8c3".to_string(),
            "eventProcessor": "substrate_event_processor".to_string()
        });

        main(action).unwrap();
    }
}
