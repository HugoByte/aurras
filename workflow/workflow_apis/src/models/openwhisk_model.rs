use actix_extract_multipart::*;
use serde::{Deserialize, Serialize};

// Input struct for creating an action
#[derive(Debug, Deserialize)]
pub struct ActionInput {
    pub name: String,
    pub kind: String,
    #[serde(default = "default_resource")]
    pub image: String,
    pub file: File,
    pub url: String,
    pub namespace: String,
    pub auth: String,
}

// Input for deleting an action, trigger or rule
#[derive(Debug, Deserialize)]
pub struct Delete {
    pub name: String,
    pub url: String,
    pub namespace: String,
    pub auth: String,
    pub deleting_type: String,
}

// Input for creating an trigger
#[derive(Debug, Deserialize, Serialize)]
pub struct TriggerInput {
    pub name: String,
    // pub annotation: String,
    #[serde(default)]
    pub param_json: String,
    pub url: String,
    pub namespace: String,
    pub auth: String,
    pub rule: String,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub url: String,
    pub namespace: String,
    pub auth: String,
    pub list_type: String,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct ActionList {
    pub name: String,
    pub namespace: String,
}

fn default_resource() -> String {
    "openwhisk/action-rust-v1.34".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRule {
    pub rule: String,
    pub active_status: String,
}
