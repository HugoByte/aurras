use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExecutionState {
    #[default]
    Init,
    Running,
    Paused,
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

            ExecutionState::Init | ExecutionState::Paused => {
                self.execution_state = ExecutionState::Running;
                Ok(())
            }
        }
    }

    fn update_result(&mut self, result: Value, is_success: bool) -> Result<()> {
        match self.execution_state {
            ExecutionState::Failed | ExecutionState::Success => Err(anyhow!(
                "workflow already executed! result: {:?}",
                self.get_result()
            )),

            ExecutionState::Init => {
                Err(anyhow!("workflow does not executed! execution_state: Init"))
            }

            ExecutionState::Paused => {
                Err(anyhow!("execution is paused! workflow should be resumed"))
            }

            ExecutionState::Running => {
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
            ExecutionState::Paused => Err(anyhow!("execution paused")),
            ExecutionState::Failed => Ok(false),
            ExecutionState::Success => Ok(true),
        }
    }

    fn get_result(&self) -> Result<Value> {
        match self.execution_state {
            ExecutionState::Init => Err(anyhow!("execution not started")),
            ExecutionState::Running => Err(anyhow!("execution in-progress")),
            ExecutionState::Paused => match &self.result {
                Some(value) => Ok(value.clone()),
                None => Err(anyhow!("no result is stored!")),
            },
            ExecutionState::Failed | ExecutionState::Success => Ok(self.result.clone().unwrap()),
        }
    }
}
