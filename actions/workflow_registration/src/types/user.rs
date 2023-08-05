use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRegistration {
    workflow_name: String,
    version: String,
    kind: String,
    file: String,
    auth_token: String,
}

#[allow(dead_code)]
impl WorkflowRegistration {
    pub fn new(
        workflow_name: String,
        version: String,
        kind: String,
        file: String,
        auth_token: String,
    ) -> Self {
        WorkflowRegistration {
            workflow_name,
            version,
            kind,
            file,
            auth_token,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.workflow_name
    }
    pub fn get_file(&self) -> &String {
        &self.file
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WorkflowDetails {
    #[serde(skip_serializing, rename(deserialize = "_id"))]
    pub id: String,
    #[serde(skip_serializing, rename(deserialize = "_rev"))]
    pub rev: String,
    pub action_list: Vec<String>,
}
