use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub rev: String,
    pub data: Vec<UserData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserData {
    pub user_id: String,
    pub status: bool,
    pub input_data: serde_json::Value,
}
