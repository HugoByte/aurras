use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub topic: String,
    pub token: String,
    pub address: String
}
