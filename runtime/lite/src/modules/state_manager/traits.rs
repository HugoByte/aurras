use super::*;

pub trait WorkflowStateManager{
    fn update_running(&mut self) -> Result<()>;
    fn update_paused(&mut self, output: Option<Value>) -> Result<()>;
    fn update_result(&mut self, result: Value, is_success: bool) -> Result<()>;
    fn get_id(&self) -> usize;
    fn get_workflow_name(&self) -> String;
    fn get_execution_state(&self) -> ExecutionState;
    fn is_success(&self) -> Result<bool>;
    fn get_result(&self) -> Result<Value>;
}

pub trait GlobalStateManager {
    fn new_workflow(&mut self, workflow_id: usize, workflow_name: &str) -> usize; // returns index(used as id also)
    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>>;
    fn update_running(&mut self, workflow_index: usize) -> Result<()>;
    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()>;
    fn update_result(&mut self, workflow_index: usize, result:Value, is_success: bool, is_cached: bool) -> Result<()>;
}
