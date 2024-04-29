use super::*;

// event_logs structure
/// The `TxEventLog` struct represents event logs associated with a transaction in Rust.
/// 
/// Properties:
/// 
/// * `score_address`: The `score_address` property in the `TxEventLog` struct represents the address of
/// a smart contract or Score (Smart Contract On Reliable Environment) in a blockchain network. It is a
/// unique identifier for the smart contract on the blockchain.
/// * `indexed_events`: The `indexed_events` property in the `TxEventLog` struct represents a vector of
/// strings that contains indexed events related to the transaction event log. These events are
/// typically used for efficient searching and filtering of the transaction logs based on specific
/// criteria.
/// * `data`: The `data` property in the `TxEventLog` struct is a vector of strings. It seems to store
/// multiple strings of data related to the transaction event log.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TxEventLog {
    #[serde(rename = "scoreAddress")]
    pub score_address: String,
    #[serde(rename = "indexed")]
    pub indexed_events: Vec<String>,
    pub data: Vec<String>,
}
