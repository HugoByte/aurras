use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDelete {
    __ow_method: String,
    pub url: String,
    pub namespace: String,
    pub auth: String,
    pub action_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowDetails {
    pub action_name: String,
    pub trigger_name: String,
    pub rule_name: String,
}
