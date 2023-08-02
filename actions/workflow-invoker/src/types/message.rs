use serde_derive::{Deserialize, Serialize};

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
