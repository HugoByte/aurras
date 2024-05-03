use super::*;
use crate::modules::state_manager;

/// Represents the output of the workflow. A wrapper around the type `serde_json::Value`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

/// Represents the input of the workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    pub allowed_hosts: Option<Vec<String>>,
    pub data: Value,
}

/// Represents the internal state of the workflow.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InternalState {
    pub action_name: String,
    pub task_index: isize,
    pub execution_state: state_manager::ExecutionState,
    pub output: Option<Value>,
    pub error: Option<String>,
}
