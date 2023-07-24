use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbDatas {
    pub endpoint: String,
    pub validator: String,
    pub key: String,
}
