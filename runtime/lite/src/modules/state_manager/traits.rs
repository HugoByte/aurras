use super::*;

/// Defines the `WorkflowStateManager` trait. This trait contains methods for managing the state of a workflow.
pub trait WorkflowStateManager {
    /// Updates the execution state of the workflow to `Running`.
    fn update_running(&mut self) -> Result<()>;

    /// Updates the execution state of the workflow to `Success` or `Failed` and sets the result.
    /// 
    /// # Arguments
    /// 
    /// * `result`: The result of the workflow.
    /// * `is_success`: A boolean value that indicates whether the workflow was successful or not.
    fn update_result(&mut self, result: Value, is_success: bool) -> Result<()>;
    
    /// Updates the execution state of the workflow to `Paused`.
    /// 
    /// # Arguments
    /// 
    /// * `output`: An optional value that represents the output of the workflow.
    fn update_paused(&mut self, output: Option<Value>) -> Result<()>;

    /// Returns the ID of the workflow.
    fn get_id(&self) -> String;

    /// Returns the name of the workflow.
    fn get_workflow_name(&self) -> String;

    /// Returns the execution state of the workflow.
    fn get_execution_state(&self) -> ExecutionState;

    /// Returns the result of the workflow as a `bool`.
    fn is_success(&self) -> Result<bool>;

    /// Returns the result of the workflow.
    fn get_result(&self) -> Result<Value>;

}

/// Defines the `GlobalStateManager` trait. This trait contains methods of the global state manager,
/// which manages the state of all workflows.
pub trait GlobalStateManager {
    /// Creates a new workflow to the state manager and returns its index.
    ///
    /// # Arguments
    ///
    /// * `workflow_id`: ID of the workflow.
    /// * `workflow_name`: Name of the workflow.
    ///
    /// # Returns
    ///
    /// Returns the index of the newly created workflow.
    fn new_workflow(&mut self, workflow_id: &str, workflow_name: &str) -> usize;

    /// Returns the state data of the workflow with the given index.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: Index of the workflow.
    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>>;

    /// Updates the state data of the workflow using the given index. This method updates the state to `Running`.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: Index of the workflow.
    fn update_running(&mut self, workflow_index: usize) -> Result<()>;

    /// Updates the state data of the workflow using the given index. This method updates the state to `Paused`.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: Index of the workflow.
    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()>;

    /// Updates the state data of the workflow using the given index. This method updates the state to
    /// `Success` or `Failed`.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: Index of the workflow.
    /// * `result`: Result of the workflow in JSON format.
    /// * `is_success`: A boolean value that indicates whether the workflow was successful or not.
    fn update_result(
        &mut self,
        workflow_index: usize,
        result: Value,
        is_success: bool,
    ) -> Result<()>;
}
