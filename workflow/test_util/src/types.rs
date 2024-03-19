use super::*;

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

#[allow(unused)]
#[derive(Default, Debug)]
pub struct StateManager{
    action_name: String,
    task_index: isize,
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>,
}

#[allow(unused)]
impl StateManager {

    fn update_state_data(&self) {
        todo!()
    }

    pub fn init() -> Self {
        todo!()
    }

    pub fn update_workflow_initialized(&mut self) {
        todo!()
    }

    pub fn update_running(&mut self, action_name: &str, task_index: isize) {
        todo!()
    }

    pub fn update_pause(&mut self) {
        todo!()
    }

    pub fn update_success(&mut self, output: Value) {
        todo!()
    }

    pub fn update_restore_success(&mut self, action_name: &str, task_index: isize, output: Value) {
        todo!()
    }

    pub fn update_err(&mut self, error: &str) {
        todo!()
    }

}
