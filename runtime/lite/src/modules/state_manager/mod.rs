use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod traits;
pub mod types;
pub use logger::{CoreLogger, Logger};
pub use traits::*;
pub use types::*;

use super::logger;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct GlobalState<T: WorkflowStateManager, U: Logger> {
    workflows: Vec<T>,
    logger: U,
}

impl<U: Logger> GlobalState<WorkflowState, U> {
    pub fn new(logger: U) -> Self {
        Self {
            workflows: Vec::<WorkflowState>::new(),
            logger,
        }
    }
}

impl<U: Logger> GlobalStateManager for GlobalState<WorkflowState, U> {
    fn new_workflow(&mut self, workflow_id: usize, workflow_name: &str) -> usize {
        self.workflows
            .push(WorkflowState::new(workflow_id, workflow_name));
        self.logger
            .info(&format!("[new workflow created with id:{}]", workflow_id));
        self.workflows.len() - 1
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
            self.logger.info(&format!(
                "[workflow:{} starting...]",
                self.workflows[workflow_index].get_id()
            ));
            self.workflows[workflow_index].update_running()?;
            self.logger.info(&format!(
                "[workflow:{} running]",
                self.workflows[workflow_index].get_id()
            ));
            Ok(())
        }
    }

    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_paused(output)?;
            self.logger.warn(&format!(
                "[workflow:{} paused]",
                self.workflows[workflow_index].get_id()
            ));
            Ok(())
        }
    }

    fn update_result(
        &mut self,
        workflow_index: usize,
        result: Value,
        is_success: bool,
    ) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_result(result.clone(), is_success)?;

            if is_success {
                self.logger.info(&format!(
                    "[workflow:{} execution success✅]",
                    self.workflows[workflow_index].get_id()
                ));
            } else {
                let id = self.workflows[workflow_index].get_id();
                self.logger
                    .error(&format!("[workflow:{} execution failed❌]", id));
                let result: String =
                    serde_json::from_value(result.get("Err").unwrap().clone()).unwrap();
                self.logger
                    .error(&format!("[workflow:{} result: {}]", id, result));
            }

            Ok(())
        }
    }
}
