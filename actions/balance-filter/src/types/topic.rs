use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(skip_serializing, rename(deserialize = "_id"))]
    pub id: String,
    #[serde(skip_serializing, rename(deserialize = "_rev"))]
    pub rev: String,
    pub filters: HashMap<String, Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub token: String,
    pub trigger: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row<T> {
    rows: Vec<View<T>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View<T> {
    doc: T,
}
