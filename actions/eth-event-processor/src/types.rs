use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub address: String,
    pub topics: Vec<String>,
    pub event: String,
    #[serde(rename = "eventSignature")]
    pub event_signature: String,
    pub data: String,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "_isIndexed")]
    pub is_indexed: bool,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BigNumber {
    pub hex: String,
}
