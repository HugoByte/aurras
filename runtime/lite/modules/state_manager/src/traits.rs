use super::*;

pub trait WorkflowStateManager{
    fn update_running(&mut self) -> Result<()>;
    fn update_result(&mut self, result: Result<Value, String>) -> Result<()>;
    fn get_id(&self) -> usize;
    fn get_workflow_name(&self) -> String;
    fn get_execution_state(&self) -> ExecutionState;
    fn is_success(&self) -> Result<bool>;
    fn get_result(&self) -> Result<Value>;
}

pub trait GlobalStateManager {
    fn new_workflow(&mut self, workflow_id: usize, workflow_name: &str); // returns index(used as id also)
    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>>;
    fn update_execution(&mut self,workflow_index: usize) -> Result<()>;
    fn update_result(&mut self, workflow_index: usize, result: Result<Value, String>) -> Result<()>;
}
