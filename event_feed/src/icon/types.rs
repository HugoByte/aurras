use super::*;

// event_logs structure
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TxEventLog {
    #[serde(rename = "scoreAddress")]
    pub score_address: String,
    #[serde(rename = "indexed")]
    pub indexed_events: Vec<String>,
    pub data: Vec<String>,
}
