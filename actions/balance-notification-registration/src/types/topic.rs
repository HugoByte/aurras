use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub rev: String,
    pub filters: HashMap<String, Address>
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub token: String
}