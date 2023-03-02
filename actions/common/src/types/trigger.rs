use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    pub name: String,
    pub url: String
}

impl Trigger {
    pub fn new(name: String, url: String) -> Self {
        Trigger{ name, url }
    }
}