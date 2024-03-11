use anyhow::anyhow;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Init,
    Running,
    Aborted,
    Success,
}

impl Default for ExecutionState {
    fn default() -> Self {
        ExecutionState::Init
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    workflow_id: usize,
    workflow_name: String,
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>,
}

impl WorkflowStateManager for WorkflowState{
    fn get_id(&self) -> usize {
        self.workflow_id
    }

    fn get_workflow_name(&self) -> String {
        self.workflow_name.clone()
    }

    fn get_execution_state(&self) -> ExecutionState {
        self.execution_state.clone()
    }

    fn is_success(&self) -> Result<bool> {
        
        match self.execution_state{

            ExecutionState::Init => Err(anyhow!("execution not started")) ,
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Aborted => Ok(false),
            ExecutionState::Success => Ok(true)
        }

    }

    fn get_output(&self) -> Result<Value> {
        todo!()
    }
}
