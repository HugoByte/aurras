use super::*;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum ExecutionState {
    #[default]
    Init,
    Running,
    Paused,
    Failed,
    Success,
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct StateManager {
    action_name: String,
    task_index: isize,
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>,
}

#[allow(unused)]
impl StateManager {
    pub fn update_running(&mut self, action_name: &str, task_index: isize) {
        todo!()
    }

    pub fn update_success(&mut self, output: Value) {
        todo!()
    }

    pub fn update_err(&mut self, error: &str) {
        todo!()
    }
}
