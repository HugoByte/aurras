use super::*;

/// Defines the `WorkflowStateManager` trait. This trait contains methods for managing the state of a workflow.
pub trait WorkflowStateManager {
    /// Updates the execution state of the workflow to `Running`.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<(), anyhow::Error>` indicating success or failure of the update operation.
    fn update_running(&mut self) -> Result<()>;

    /// Updates the execution state of the workflow to `Success` or `Failed` and sets the result.
    /// 
    /// # Arguments
    /// 
    /// * `result`: An instance of type `Value` that represents the result of the workflow.
    /// * `is_success`: A boolean value that indicates whether the workflow was successful or not.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<(), anyhow::Error>` indicating success or failure of the update operation.
    fn update_result(&mut self, result: Value, is_success: bool) -> Result<()>;
    
    /// Updates the execution state of the workflow to `Paused`.
    /// 
    /// # Arguments
    /// 
    /// * `output`: An instance of type `Option<Value>` that represents the output of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<(), anyhow::Error>` indicating success or failure of the update operation. The
    /// method returns an error if the workflow is already in the `Paused` state or if the workflow is
    /// still in the `Init` or `Running` state.
    fn update_paused(&mut self, output: Option<Value>) -> Result<()>;

    /// Returns the ID of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `String` that represents the ID of the workflow.
    fn get_id(&self) -> String;

    /// Returns the name of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `String` that represents the name of the workflow.
    fn get_workflow_name(&self) -> String;

    /// Returns the execution state of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a variant of the enum `ExecutionState` that represents the execution state of the workflow.
    fn get_execution_state(&self) -> ExecutionState;

    /// Returns the result of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<bool>` that indicates whether the workflow was successful or not.
    fn is_success(&self) -> Result<bool>;

    /// Returns the result of the workflow.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Value>` that represents the result of the workflow.
    fn get_result(&self) -> Result<Value>;

}

/// Defines the `GlobalStateManager` trait. This trait contains methods of the global state manager,
/// which manages the state of all workflows.
pub trait GlobalStateManager {
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
    fn new_workflow(&mut self, workflow_id: &str, workflow_name: &str) -> usize; // returns index(used as id also)

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
    fn get_state_data(&self, workflow_index: usize) -> Result<Box<dyn WorkflowStateManager>>;

    /// Updates the state data of the workflow with the given index. This method updates the state to `Running`.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` that indicates whether the update was successful or not.
    fn update_running(&mut self, workflow_index: usize) -> Result<()>;

    /// Updates the state data of the workflow with the given index. This method updates the state to `Paused`.
    ///
    /// # Arguments
    ///
    /// * `workflow_index`: An integer of type `usize` that represents the index of the workflow.
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` that indicates whether the update was successful or not.
    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()>;

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
    ) -> Result<()>;
}
