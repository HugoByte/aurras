use super::Topic;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, Error};

// pub type Payload = Vec<(Era, Topic)>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub topic: String,
    pub value: String,
}

//TODO: Change
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Era {
    #[serde(rename = "era")]
    pub era: u32,
}

impl Message {
    pub fn parse_value(&self) -> Result<Era, Error> {
        from_str(&self.value)
    }
}
