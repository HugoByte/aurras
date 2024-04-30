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

/// Represents the global state manager which manages the state of multiple workflows.
/// 
/// # Properties
/// 
/// * `workflows`: A vector of type `T` that implements the `WorkflowState` trait. This represents the state
/// of all workflows.
/// * `logger`: An instance of type `U` that implements the `Logger` trait. This represents the logger.
#[derive(Debug)]
pub struct GlobalState<T: WorkflowStateManager, U: Logger> {
    workflows: Vec<T>,
    logger: U,
}

/// Implements the `GlobalState` struct methods.
impl<U: Logger> GlobalState<WorkflowState, U> {

    /// Creates a new `GlobalState` instance with the given logger.
    /// 
    /// # Arguments
    /// 
    /// * `logger`: An instance of type `U` that implements the `Logger` trait.
    /// 
    /// # Returns
    /// 
    /// Returns a new `GlobalState` instance.
    pub fn new(logger: U) -> Self {
        Self {
            workflows: Vec::<WorkflowState>::new(),
            logger,
        }
    }
}

/// Implements the `GlobalStateManager` trait for the `GlobalState` struct.
impl<U: Logger> GlobalStateManager for GlobalState<WorkflowState, U> {

    /// Creates a new workflow to the state manager and returns its index.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_id`: A string slice that represents the ID of the workflow.
    /// * `workflow_name`: A string slice that represents the name of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns the index of the newly created workflow.
    fn new_workflow(&mut self, workflow_id: &str, workflow_name: &str) -> usize {
        self.workflows
            .push(WorkflowState::new(workflow_id, workflow_name));
        self.logger
            .info(&format!("[new workflow created with id:{}]", workflow_id));
        self.workflows.len() - 1
    }

    /// Returns the state data of the workflow with the given index.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Box<dyn WorkflowStateManager>>` that contains the state data of the workflow
    /// or an `anyhow::Error` if the index is out of bounds.
    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            Ok(Box::new(self.workflows[workflow_index].clone()))
        }
    }

    /// Updates the state data of the workflow with the given index. This method updates the state to `Running`.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` that indicates whether the update was successful or not.
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

    /// Updates the state data of the workflow with the given index. This method updates the state to `Paused`.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` that indicates whether the update was successful or not.
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

    /// Updates the state data of the workflow with the given index. This method updates the state to `Success` or `Failed`.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    /// * `result`: A `Value` that represents the result of the workflow.
    /// * `is_success`: A boolean value that indicates whether the workflow was successful or not. If `true`, the state 
    /// will be set to `Success`. Otherwise, it will be set to `Failed`.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` that indicates whether the update was successful or not.
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
