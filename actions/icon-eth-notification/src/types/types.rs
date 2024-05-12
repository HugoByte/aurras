use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub address: String,
    pub token: String,
    pub data: String,
    pub req_id: i64,
    pub to: String,
    pub from: String,
    #[serde(default)]
    pub tx: String,
}
