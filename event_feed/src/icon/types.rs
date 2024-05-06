use super::*;

/// Represents a transaction event log of the Icon blockchain.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TxEventLog {
    #[serde(rename = "scoreAddress")]
    pub score_address: String,
    #[serde(rename = "indexed")]
    pub indexed_events: Vec<String>,
    pub data: Vec<String>,
}
