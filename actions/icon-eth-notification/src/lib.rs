extern crate serde_json;
mod types;

#[cfg(test)]
use actions_common::Config;
use actions_common::Context;
// use chesterfield::sync::{Client, Database};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};
use types::Event;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    pub data: Value,
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

    pub fn get_context(&mut self) -> &Context {
        self.context.as_mut().expect("Action not Initialized!")
    }

    pub fn invoke_push_notification(&mut self, value: &Value) -> Result<Value, Error> {
        self.get_context().invoke_action("push_notification", value)
    }

    pub fn get_keccak_hash(&self, input: String) -> String {
        let mut hasher = Sha3::keccak256();
        hasher.input_str(&input);
        let res = hasher.result_str();
        "0x".to_string() + &res
    }

    pub fn filter_event(&mut self) -> Result<Value, Error> {
        let event = serde_json::from_value::<Event>(self.params.data.clone()).unwrap();

        if event.from.starts_with("0x") {
            let address_hash = self.get_keccak_hash(event.address.clone());
            if event.from == address_hash {
                self.invoke_push_notification(
                    &serde_json::json!({
                        "token": event.token.clone(),
                        "message": {
                            "title": "Message Recieved On Hardhat Chain",
                            "body": format!("Message sent to {} Recieved with request id {}", event.to.clone(), event.req_id.clone())
                        }
                    }),
                )?;
            } else {
                return Err("Address is Not matched").map_err(serde::de::Error::custom);
            }
        } else {
            if event.address == event.from {
                self.invoke_push_notification(
                    &serde_json::json!({
                        "token": event.token.clone(),
                        "message": {
                            "title": "Message Recieved On Icon Chain",
                            "body": format!("Message sent to {} Recieved with request id {}", event.to.clone(), event.req_id.clone())
                        }
                    }),
                )?;
            } else {
                return Err("Address is Not matched").map_err(serde::de::Error::custom);
            }
        }

        Ok(serde_json::json!({
            "action": "success"
        }))
    }
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;
    let mut action = Action::new(input);

    action.filter_event()
}

#[cfg(test)]
mod tests {
    use crate::{Action, Input, main};

    #[test]
    fn test_get_keccak_hash() {
        let action = Action::new(crate::Input {
            data: serde_json::json!({}),
        });
        let result = action.get_keccak_hash(
            "btp://0x3.icon/cx2196b1de35d8e3cb40bcbf0142d29263c212ed5d".to_string(),
        );
        assert_eq!(
            "0xa1f733357b915d1ede96cec955a028f730cf796d714a5975563f8e12f0875567",
            result
        )
    }

    // #[test]
    // fn test_filter_icon_event() {
    //     let data = serde_json::json!({"address":"btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1" ,
    //         "token" : "hshfhsdhghn","data":"0x73656e6443616c6c4d6573736167655f686172646861745f69636f6e30",
    //         "to":"cxdf97f11ef352727f5d826eec68e569d6471aa9a4",
    //         "req_id":1,
    //         "from":"btp://0x539.hardhat/0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1"});
    //     let input = Input { data };
    //     let mut action = Action::new(input);
    //     action.filter_event();
    // }

    // #[test]
    // fn test_filter_hardhat_event() {
    //     let data = serde_json::json!({"address":"btp://0x3.icon/cx2196b1de35d8e3cb40bcbf0142d29263c212ed5d" ,
    //         "token" : "hshfhsdhghn",
    //         "data":"0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174",
    //         "from":"0xa1f733357b915d1ede96cec955a028f730cf796d714a5975563f8e12f0875567",
    //         "req_id":1,
    //         "to":"0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"});

    //     let input = Input { data };
    //     let mut action = Action::new(input);
    //     action.filter_event();
    // }

    // #[test]
    // fn test_filter_event() {
    //     let data = serde_json::json!({"address":"btp://0x3.icon/cx2196b1de35d8e3cb40bcbf0142d29263c212ed5d" ,
    //         "token" : "hshfhsdhghn",
    //         "data":"0x73656e6443616c6c4d6573736167655f69636f6e305f68617264686174",
    //         "from":"0xa1f733357b915d1ede96cec955a028f730cf796d714a5975563f8e12f0875567",
    //         "req_id":1,
    //         "to":"0x16f7d9e9e594e72a2e6120902a1339f4be2c2f06b73a09dc24c1956d61880911"});

    //     let input = serde_json::json!({"data": data});
        
    //    main(input);
    // }
}
