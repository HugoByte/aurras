use super::Topic;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, Error, Value};

pub trait Filter<T> {
    fn new() -> Self;
    fn add(self) -> Self;
    // Add Trait
    fn filter(self) -> Self;
}

pub type Payload = Vec<(Deposit, Topic)>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub topic: String,
    pub value: String,
}

//TODO: Change
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    #[serde(rename = "to")]
    pub address: String,
    #[serde(rename = "value")]
    pub amount: u32,
}

impl Message {
    pub fn parse_value(&self) -> Result<Deposit, Error> {
        from_str(&self.value)
    }
}
