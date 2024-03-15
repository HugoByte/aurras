use super::*;
use crate::WorkflowGraph;
use core::default;
// pub use logger::{Logger, CoreLogger};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ExecutionState {
    // Init,
    Running,
    Paused,
    Aborted,
    Success,
}

impl Default for ExecutionState {
    fn default() -> Self {
        ExecutionState::Running
    }
}

#[derive(Default, Debug)]
// pub struct StateManager<T: Logger> {
pub struct StateManager{
    // execution_state: ExecutionState, // to represent the task life cycle
    action_name: String,   // task name
    task_index: isize,     // n'th task out of m tasks
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>, // to define the error kind
    // logger: T,
}

// impl StateManager<CoreLogger> {
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
            execution_state: ExecutionState::Running,
            output: None,
            task_index: -1,
            error: None,
            // logger: CoreLogger::new(None),
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
        // self.logger.info(&format!("[task index:{}, action:{} running]", task_index, action_name));
        self.update_state_data();
    }

    pub fn update_pause(&mut self) {
        self.execution_state = ExecutionState::Paused;
        // self.logger.info(&format!("[task index:{}, action:{} paused]", self.task_index, self.action_name));
        self.update_state_data();
    }

    pub fn update_success(&mut self, output: Value) {
        self.output = Some(output);
        self.execution_state = ExecutionState::Success;
        // self.logger.info(&format!("[task index:{}, action:{} success]", self.task_index, self.action_name));
        self.update_state_data();
    }

    pub fn update_restore_success(&mut self, action_name: &str, task_index: isize, output: Value) {
        self.action_name = action_name.to_string();
        self.task_index = task_index;
        self.execution_state = ExecutionState::Success;
        self.output = Some(output);
        // self.logger.info(&format!("[task index:{}, action:{} success(cached)]", task_index, action_name));
        self.update_state_data();
    }

    pub fn update_err(&mut self, error: &str) {
        self.execution_state = ExecutionState::Aborted;
        self.error = Some(error.to_string());
        // self.logger.error(&format!("[task index:{}, action:{} aborted]", self.task_index, self.action_name));
        self.update_state_data();
    }

}
