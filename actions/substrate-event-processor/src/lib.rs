extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    section: String,
    method: String,
    data: Vec<HashMap<String, String>>,
}

struct Action {
    params: Input,
}

impl Action {
    pub fn new(params: Input) -> Self {
        Action { params }
    }

    pub fn parse_event_data(&self) -> Result<Value, Error> {
        return match self.params.section.as_str() {
            "balances" => {
                return match self.params.method.as_str() {
                    "Transfer" => Ok(serde_json::json!({
                        "to": self.params.data[1].get("AccountId").unwrap(),
                        "value": format!("{:#.4}", self.params.data[2].get("Balance").unwrap().parse::<f64>().unwrap() / u64::pow(10,10) as f64),
                    })),
                    _ => Ok(serde_json::json!({}))
                }
            },
            _ => Ok(serde_json::json!({}))
        }
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args).unwrap();
    let action = Action::new(input);
    action.parse_event_data()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_data_pass() {
        
        let input = serde_json::from_value::<Input>(serde_json::json!({
            "section": "balances",
            "method": "Transfer",
            "meta": "[ Transfer succeeded. \\[from, to, value\\]]",
            "data": [
              { "AccountId": "13sc83poXh93CXtzNjaCwo2Q88cS9oNyJ6Ru7DyxchqKVbbc" },
              { "AccountId": "1N55WJHup5j1LHpbzQX6zvYu7QeLcUBd1tBp8CvA7xHGixY" },
              { "Balance": "731000000000" }
            ]
          })).unwrap();
        
        let action = Action::new(input);

        let response = action
            .parse_event_data()
            .unwrap();

        assert_eq!(response, serde_json::json!({"to": "1N55WJHup5j1LHpbzQX6zvYu7QeLcUBd1tBp8CvA7xHGixY", "value": "73.1000"}));
    }
}