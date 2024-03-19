use super::*;
use crate::WorkflowGraph;
use core::default;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ExecutionState {
    Init,
    Running,
    Paused,
    Failed,
    Success,
}

impl Default for ExecutionState {
    fn default() -> Self {
        ExecutionState::Running
    }
}

#[derive(Default, Debug)]
pub struct StateManager{
    action_name: String,
    task_index: isize,
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>,
}

impl StateManager {

    fn update_state_data(&self) {
        let state_data: serde_json::Value = serde_json::json!(
            {
                "action_name": self.action_name,
                "task_index": self.task_index,
                "execution_state": self.execution_state,
                "output":   self.output,
                "error":   self.error
            }
        );

        let serialized = serde_json::to_vec(&state_data).unwrap();
        let size = serialized.len() as i32;
        let ptr = serialized.as_ptr();

        std::mem::forget(ptr);

        unsafe {
            super::set_state(ptr as i32, size);
        }
    }

    pub fn init() -> Self {
        let state_data = StateManager {
            action_name: "Initializing Workflow".to_string(),
            execution_state: ExecutionState::Init,
            output: None,
            task_index: -1,
            error: None,
        };

        state_data.update_state_data();
        state_data
    }

    pub fn update_workflow_initialized(&mut self) {
        self.execution_state = ExecutionState::Success;
        self.task_index = -1;
        self.error = None;
        self.update_state_data();
    }

    pub fn update_running(&mut self, action_name: &str, task_index: isize) {
        self.action_name = action_name.to_string();
        self.task_index = task_index;
        self.execution_state = ExecutionState::Running;
        self.output = None;
        self.update_state_data();
    }

    pub fn update_pause(&mut self) {
        self.execution_state = ExecutionState::Paused;
        self.update_state_data();
    }

    pub fn update_success(&mut self, output: Value) {
        self.output = Some(output);
        self.execution_state = ExecutionState::Success;
        self.update_state_data();
    }

    pub fn update_restore_success(&mut self, action_name: &str, task_index: isize, output: Value) {
        self.action_name = action_name.to_string();
        self.task_index = task_index;
        self.execution_state = ExecutionState::Success;
        self.output = Some(output);
        self.update_state_data();
    }

    pub fn update_err(&mut self, error: &str) {
        self.execution_state = ExecutionState::Failed;
        self.error = Some(error.to_string());
        self.update_state_data();
    }

}
