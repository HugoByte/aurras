use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    name: String,
    topic: String,
    trigger: String
}

impl Source {
    pub fn new(name: String, topic: String, trigger: String) -> Self {
        Source { name, topic, trigger }
    }
}
