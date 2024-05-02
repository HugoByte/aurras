use super::*;

/// Represents the execution state of a workflow. This enum is used to track the lifecycle of a workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionState {
    #[default]
    Init,
    Running,
    Paused,
    Failed,
    Success,
}

/// Represents the state of a workflow in the execution.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowState {
    workflow_id: String,
    workflow_name: String,
    execution_state: ExecutionState,
    result: Option<Value>,
}

impl WorkflowState {

    /// Creates a new `WorkflowState` instance which is used to represent the state data of a workflow.
    /// 
    /// # Arguments
    /// 
    /// * `workflow_id`: Unique identifier of a workflow.
    /// * `workflow_name`: Name of the workflow.
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            execution_state: ExecutionState::Init,
            result: None,
        }
    }
}

impl WorkflowStateManager for WorkflowState {

    /// Updates the execution state of the workflow to `Running`.
    fn update_running(&mut self) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Running => Err(anyhow!("workflow execution already in progress!")),

            ExecutionState::Init | ExecutionState::Paused => {
                self.execution_state = ExecutionState::Running;
                Ok(())
            }
        }
    }

    /// Updates the execution state of the workflow to `Success` or `Failed` and sets the result.
    /// 
    /// # Arguments
    /// 
    /// * `result`: The result of the workflow.
    /// * `is_success`: A boolean value that indicates whether the workflow was successful or not.
    fn update_result(&mut self, result: Value, is_success: bool) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Init => {
                Err(anyhow!("workflow does not executed! execution_state: Init"))
            }

            ExecutionState::Running | ExecutionState::Paused => {
                if is_success {
                    self.execution_state = ExecutionState::Success;
                } else {
                    self.execution_state = ExecutionState::Failed;
                }
                self.result = Some(result);
                Ok(())
            }
        }
    }

    /// Updates the execution state of the workflow to `Paused`.
    /// 
    /// # Arguments
    /// 
    /// * `output`: An optional value that represents the output of the workflow.
    fn update_paused(&mut self, output: Option<Value>) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Paused => Err(anyhow!("workflow already paused")),

            ExecutionState::Init | ExecutionState::Running => {
                self.execution_state = ExecutionState::Paused;

                if output.is_some() {
                    self.result = output
                }

                Ok(())
            }
        }
    }

    /// Returns the ID of the workflow.
    fn get_id(&self) -> String {
        self.workflow_id.clone()
    }

    /// Returns the name of the workflow.
    fn get_workflow_name(&self) -> String {
        self.workflow_name.clone()
    }

    /// Returns the execution state of the workflow.
    fn get_execution_state(&self) -> ExecutionState {
        self.execution_state.clone()
    }

    /// Returns the result of the workflow as a `bool`.
    fn is_success(&self) -> Result<bool> {
        match self.execution_state {
            ExecutionState::Init => Err(anyhow!("execution not started")),
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Paused => Err(anyhow!("execution paused")),
            ExecutionState::Failed => Ok(false),
            ExecutionState::Success => Ok(true),
        }
    }

    /// Returns the result of the workflow.
    fn get_result(&self) -> Result<Value> {
        match self.execution_state {
            ExecutionState::Init => Err(anyhow!("execution not started")),
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Paused => match &self.result {
                Some(value) => Ok(value.clone()),
                None => Err(anyhow!("no result is stored!")),
            },
            ExecutionState::Failed | ExecutionState::Success => self
                .result
                .clone()
                .ok_or_else(|| anyhow!("no result available for completed workflow")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Init);
        assert_eq!(workflow_state.result, None);
    }

    #[test]
    fn test_update_running_pass_from_init() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Running);
        assert_eq!(workflow_state.result, None);
    }

    #[test]
    fn test_update_running_pass_from_paused() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_paused(None).unwrap();
        workflow_state.update_running().unwrap();
        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Running);
        assert_eq!(workflow_state.result, None);
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Null)"]
    fn test_update_running_fail_already_failed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.update_result(Value::Null, false).unwrap();
        workflow_state.update_running().unwrap();
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Null)"]
    fn test_update_running_fail_already_success() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.update_result(Value::Null, true).unwrap();
        workflow_state.update_running().unwrap();
    }

    #[test]
    #[should_panic = "workflow execution already in progress!"]
    fn test_update_running_fail_already_running() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.update_running().unwrap();
    }

    #[test]
    fn test_update_result_pass_from_running() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_result(data.clone(), true).unwrap();

        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Success);
        assert_eq!(workflow_state.result, Some(data));
    }

    #[test]
    fn test_update_result_pass_from_paused() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_paused(Some(data.clone())).unwrap();
        workflow_state.update_result(data.clone(), false).unwrap();

        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Failed);
        assert_eq!(workflow_state.result, Some(data));
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Object {\"result\": String(\"failed\")})"]
    fn test_update_result_fail_already_failed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "failed"});
        workflow_state.update_result(data.clone(), false).unwrap();

        workflow_state.update_result(data.clone(), false).unwrap();
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Object {\"result\": String(\"success\")})"]
    fn test_update_result_fail_already_success() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_result(data.clone(), true).unwrap();

        workflow_state.update_result(data.clone(), false).unwrap();
    }

    #[test]
    #[should_panic = "workflow does not executed! execution_state: Init"]
    fn test_update_result_fail_not_executed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_result(data.clone(), false).unwrap();
    }

    #[test]
    fn test_update_paused_pass_from_init() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_paused(Some(data.clone())).unwrap();
        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Paused);
        assert_eq!(workflow_state.result, Some(data));
    }

    #[test]
    fn test_update_paused_pass_from_running() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_paused(Some(data.clone())).unwrap();
        assert_eq!(workflow_state.workflow_id, 1.to_string());
        assert_eq!(workflow_state.workflow_name, "test_workflow");
        assert_eq!(workflow_state.execution_state, ExecutionState::Paused);
        assert_eq!(workflow_state.result, Some(data));
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Object {\"result\": String(\"success\")})"]
    fn test_update_paused_fail_already_success() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "success"});
        workflow_state.update_result(data.clone(), true).unwrap();
        workflow_state.update_paused(Some(data.clone())).unwrap();
    }

    #[test]
    #[should_panic = "workflow already executed! result: Ok(Object {\"result\": String(\"failed\")})"]
    fn test_update_paused_fail_already_failed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data: Value = serde_json::json!({"result": "failed"});
        workflow_state.update_result(data.clone(), false).unwrap();
        workflow_state.update_paused(Some(data.clone())).unwrap();
    }

    #[test]
    #[should_panic = "workflow already paused"]
    fn test_update_paused_fail_already_paused() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_paused(None).unwrap();
        workflow_state.update_paused(None).unwrap();
    }

    #[test]
    fn test_get_id_pass() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        assert_eq!(workflow_state.get_id(), String::from("1"));
    }

    #[test]
    fn test_get_workflow_name_pass() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        assert_eq!(workflow_state.get_workflow_name(), "test_workflow");
    }

    #[test]
    fn test_get_execution_state_pass() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        assert_eq!(workflow_state.get_execution_state(), ExecutionState::Init);
    }

    #[test]
    fn test_is_success_pass_success() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.update_result(Value::Null, true).unwrap();
        let is_success = workflow_state.is_success().unwrap();
        assert_eq!(is_success, true);
    }

    #[test]
    fn test_is_success_pass_failed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.update_result(Value::Null, false).unwrap();
        let is_success = workflow_state.is_success().unwrap();
        assert_eq!(is_success, false);
    }

    #[test]
    #[should_panic = "execution not started"]
    fn test_is_success_fail_not_executed() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.is_success().unwrap();
    }

    #[test]
    #[should_panic = "execution in-progress"]
    fn test_is_success_fail_not_completed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.is_success().unwrap();
    }

    #[test]
    #[should_panic = "execution paused"]
    fn test_is_success_fail_paused() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_paused(None).unwrap();
        workflow_state.is_success().unwrap();
    }

    #[test]
    fn test_get_result_pass_success_result() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        let data = Value::String("success".to_string());
        workflow_state.update_result(data.clone(), true).unwrap();
        let result = workflow_state.get_result().unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_get_result_pass_paused_result() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        let data = Value::String("success".to_string());
        workflow_state.update_paused(Some(data.clone())).unwrap();
        let result = workflow_state.get_result().unwrap();
        assert_eq!(result, data);
    }

    #[test]
    #[should_panic = "execution not started"]
    fn test_get_result_fail_not_executed() {
        let workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.get_result().unwrap();
    }

    #[test]
    #[should_panic = "execution in-progress"]
    fn test_get_result_fail_not_completed() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_running().unwrap();
        workflow_state.get_result().unwrap();
    }

    #[test]
    #[should_panic = "no result is stored!"]
    fn test_get_result_fail_paused_no_result() {
        let mut workflow_state = WorkflowState::new("1", "test_workflow");
        workflow_state.update_paused(None).unwrap();
        workflow_state.get_result().unwrap();
    }
}
