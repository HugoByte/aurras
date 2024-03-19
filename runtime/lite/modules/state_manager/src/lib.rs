mod traits;
mod types;

use anyhow::anyhow;
pub use traits::*;
pub use types::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState<T: WorkflowStateManager> {
    workflows: Vec<T>,
}

impl GlobalState<WorkflowState> {
    pub fn new() -> Self {
        Self {
            workflows: Vec::<WorkflowState>::new(),
        }
    }
}

impl GlobalStateManager for GlobalState<WorkflowState> {
    fn new_workflow(&mut self, workflow_id: usize, workflow_name: &str) {
        self.workflows
            .push(WorkflowState::new(workflow_id, workflow_name));
    }

    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            Ok(Box::new(self.workflows[workflow_index].clone()))
        }
    }

    fn update_running(&mut self, workflow_index: usize) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_running()?;
            Ok(())
        }
    }

    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_paused(output)?;
            Ok(())
        }
    }

    fn update_result(
        &mut self,
        workflow_index: usize,
        result: Result<Value, String>,
    ) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_result(result)?;
            Ok(())
        }
    }
}
