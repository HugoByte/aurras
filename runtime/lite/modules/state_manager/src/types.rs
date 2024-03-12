use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExecutionState {
    #[default]
    Init,
    Running,
    Failed,
    Success,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    workflow_id: usize,
    workflow_name: String,
    execution_state: ExecutionState,
    result: Option<Value>,
}

impl WorkflowState {
    pub fn new(workflow_id: usize, workflow_name: &str) -> Self {
        Self {
            workflow_id,
            workflow_name: workflow_name.to_string(),
            execution_state: ExecutionState::Init,
            result: None,
        }
    }
}

impl WorkflowStateManager for WorkflowState {
    fn update_running(&mut self) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Running => Err(anyhow!("workflow execution already in progress!")),

            ExecutionState::Init => {
                self.execution_state = ExecutionState::Running;
                Ok(())
            }
        }
    }

    fn update_result(&mut self, result: Result<Value, String>) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Init => {
                Err(anyhow!("workflow does not executed! execution_state: Init"))
            }

            ExecutionState::Running => {
                match result {
                    Ok(output) => {
                        self.execution_state = ExecutionState::Success;
                        self.result = Some(output);
                    }
                    Err(error) => {
                        self.execution_state = ExecutionState::Failed;
                        self.result = Some(serde_json::json!({ "error": error}));
                    }
                }
                Ok(())
            }
        }
    }

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
        match self.execution_state {
            ExecutionState::Init => Err(anyhow!("execution not started")),
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Failed => Ok(false),
            ExecutionState::Success => Ok(true),
        }
    }

    fn get_result(&self) -> Result<Value> {
        match self.execution_state {
            ExecutionState::Init => Err(anyhow!("execution not started")),
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Failed | ExecutionState::Success => Ok(self.result.clone().unwrap()),
        }
    }
}
