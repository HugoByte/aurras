use super::*;
use crate::modules::state_manager;

/// Represents the output of the workflow. A wrapper around the type `serde_json::Value`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

/// Represents the input of the workflow.
/// 
/// # Properties
/// 
/// * `allowed_hosts`: An optional vector of string that represents the host-names that are allowed to accessed
/// by the workflow.
/// * `data`: A `serde_json::Value` that represents the data that is passed to the workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    pub allowed_hosts: Option<Vec<String>>,
    pub data: Value,
}

/// Represents the internal state of the workflow. These internal state data will be returned to the
/// client when the workflow is executed.
/// 
/// # Properties
/// 
/// * `action_name`: A `String` that represents the name of the action that is being executed.
/// * `task_index`: An `isize` that represents the index of the task that is being executed. This index is used
/// to determine the order of the tasks in the workflow.
/// * `execution_state`: An instance of `ExecutionState` that represents the current state of the workflow.
/// * `output`: An `Option<Value>` that represents the output of the workflow. If the workflow is stopped before
/// execution of the task, this `output` will be `None`.
/// * `error`: An `Option<String>` that represents the error that occurred during the execution of the workflow.
/// If the execution of the task is successful, this `error` will be `None`.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InternalState {
    pub action_name: String,
    pub task_index: isize,
    pub execution_state: state_manager::ExecutionState,
    pub output: Option<Value>,
    pub error: Option<String>,
}
