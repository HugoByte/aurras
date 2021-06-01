use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    name: String,
    url: String
}

impl Trigger {
    pub fn new(name: String, url: String) -> Self {
        Trigger{ name, url }
    }
}