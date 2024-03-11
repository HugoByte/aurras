use super::*;

pub trait WorkflowStateManager {
    fn get_id(&self) -> usize;
    fn get_workflow_name(&self) -> String;
    fn get_execution_state(&self) -> ExecutionState;
    fn is_success(&self) -> Result<bool>;
    fn get_output(&self) -> Result<Value>;
}

pub trait GlobalStateManager {
    fn new_workflow(workflow_name: &str) -> usize; // returns index(used as id also)
    fn get_state_data(&self, workflow_index: usize) -> WorkflowState;
    fn update_execution(&mut self,workflow_index: usize) -> Result<()>;
    fn update_result(&mut self, workflow_index: usize) -> Result<()>;
}
