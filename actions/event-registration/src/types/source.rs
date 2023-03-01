use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    name: String,
    trigger: String
}

impl Source {
    pub fn new(name: String, trigger: String) -> Self {
        Source { name, trigger }
    }
}
