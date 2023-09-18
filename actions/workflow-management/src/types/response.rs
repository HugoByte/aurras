use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    doc: Doc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Doc {
    name: String,
    trigger: String,
}
