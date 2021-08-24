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
    event_producer: String,
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
        let db = self.connect_db(&"".to_string(), &"".to_string());
        self.context = Some(Context::new(db, Some(config)));
    }

    #[cfg(not(test))]
    pub fn init(&mut self) {
        let db = self.connect_db(&"".to_string(), &"".to_string());
        self.context = Some(Context::new(db, None));
    }

    fn connect_db(&self, db_url: &str, db_name: &str) -> Database {
        let client = Client::new(db_url).unwrap();
        let db = client.database(db_name).unwrap();
        db
    }

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn process_event(&mut self) -> Result<Value, Error> {
        let event_processor = self.params.event_processor.clone();
        let event = self.params.event.clone();
        self.get_context().invoke_action(
            &event_processor,
            &serde_json::from_str(&event)?,
        )        
    }

    pub fn invoke_action(&mut self, event: Value) -> Result<Value, Error> {
        let event_producer = self.params.event_producer.clone();
        let topic = self.params.topic.clone();
        let brokers = self.params.brokers.clone();
        self.get_context().invoke_action(
            &event_producer,
            &serde_json::json!({
                "topic": topic,
                "value": event,
                "brokers": brokers
            }),
        )
    }
}
pub fn main(args: Value) -> Result<Value, Error> {
    // TODO: Use processor for each event source to process event to generic format as the event receiver will be generic for all event source
    let input = serde_json::from_value::<Input>(args).unwrap();
    let mut action = Action::new(input);

    #[cfg(not(test))]
    action.init();
    let processed_event = action.process_event()?;
    println!("{}", processed_event);
    action.invoke_action(processed_event)
}