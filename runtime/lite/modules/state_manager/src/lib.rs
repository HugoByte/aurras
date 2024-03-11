mod types;
mod traits;

pub use types::*;
pub use traits::*; 

use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::Result;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    workflows: Vec<WorkflowState>,
}

impl GlobalStateManager for GlobalState{
    fn new_workflow(workflow_name: &str) -> usize {
        todo!()
    }
    
    fn get_state_data(&self, workflow_index: usize) -> WorkflowState {
        todo!()
    }
    
    fn update_execution(&mut self,workflow_index: usize) -> Result<()> {
        todo!()
    }
    
    fn update_result(&mut self, workflow_index: usize) -> Result<()> {
        todo!()
    }
    
}
