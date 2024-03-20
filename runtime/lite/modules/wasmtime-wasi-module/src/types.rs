use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct Resultss {
    result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    pub allowed_hosts: Option<Vec<String>>,
    pub data: Value,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InternalState{
   pub action_name: String,
   pub task_index: isize,
   pub execution_state: state_manager::ExecutionState,
   pub output: Option<Value>,
   pub error: Option<String>,
}