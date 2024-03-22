use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod traits;
pub mod types;
pub use traits::*;
pub use types::*;
pub use logger::{CoreLogger, Logger};

use super::logger;

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

impl<U: Logger> GlobalStateManager for GlobalState<WorkflowState, U>
{
    fn new_workflow(&mut self, workflow_id: usize, workflow_name: &str) {
        self.workflows
            .push(WorkflowState::new(workflow_id, workflow_name));
        self.logger.info(&format!("[new workflow created with id:{}]", workflow_id));
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
            self.logger.info(&format!("[workflow:{} starting...]", self.workflows[workflow_index].get_id()));
            self.workflows[workflow_index].update_running()?;
            self.logger.info(&format!("[workflow:{} running]", self.workflows[workflow_index].get_id()));
            Ok(())
        }
    }

    fn update_paused(&mut self, workflow_index: usize, output: Option<Value>) -> Result<()> {
        if self.workflows.len() <= workflow_index {
            Err(anyhow!("index out of bound"))
        } else {
            self.workflows[workflow_index].update_paused(output)?;
            self.logger.warn(&format!("[workflow:{} paused]", self.workflows[workflow_index].get_id()));
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

            if is_success{
                self.logger.info(&format!("[workflow:{} execution success✅]", self.workflows[workflow_index].get_id()));
            }else{
                let id = self.workflows[workflow_index].get_id();
                self.logger.error(&format!("[workflow:{} execution failed❌]", id));
                let result: String = serde_json::from_value(result.get("Err").unwrap().clone()).unwrap(); 
                self.logger.error(&format!("[workflow:{} result: {}]", id, result));
            }

            Ok(())
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_workflow() {
        let logger = CoreLogger::new(Some("./test_log_1.log"));
        let mut global_state= GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        let new_workflow_state = WorkflowState::new(0, "test_workflow");
        assert_eq!(global_state.workflows[0], new_workflow_state);
        std::fs::remove_file("./test_log_1.log").unwrap()
    }

    #[test]
    fn test_get_state_data_pass() {
        let logger = CoreLogger::new(Some("./test_log_2.log"));
        let mut global_state = GlobalState::new(logger);
        
        global_state.new_workflow(0, "test_workflow");
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_id(), 0);
        assert_eq!(state_data.get_workflow_name(), "test_workflow");
        assert_eq!(state_data.get_execution_state(), ExecutionState::Init);
        assert!(state_data.get_result().is_err());


        global_state.update_running(0).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_id(), 0);
        assert_eq!(state_data.get_workflow_name(), "test_workflow");
        assert_eq!(state_data.get_execution_state(), ExecutionState::Running);
        assert!(state_data.get_result().is_err());

        // without result
        global_state.update_paused(0, None).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_id(), 0);
        assert_eq!(state_data.get_workflow_name(), "test_workflow");
        assert_eq!(state_data.get_execution_state(), ExecutionState::Paused);
        assert!(state_data.get_result().is_err());


        global_state.update_running(0).unwrap();
        // with result
        let data = Value::String("some result".to_string());
        global_state.update_paused(0, Some(data.clone())).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_id(), 0);
        assert_eq!(state_data.get_workflow_name(), "test_workflow");
        assert_eq!(state_data.get_execution_state(), ExecutionState::Paused);
        assert_eq!(state_data.get_result().unwrap(), data);

        global_state.update_running(0).unwrap();
        global_state.update_result(0, data.clone(), true).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_id(), 0);
        assert_eq!(state_data.get_workflow_name(), "test_workflow");
        assert_eq!(state_data.get_execution_state(), ExecutionState::Success);
        assert_eq!(state_data.get_result().unwrap(), data);
        std::fs::remove_file("./test_log_2.log").unwrap()
    }

    #[test]
    #[should_panic="index out of bound"]
    fn test_get_state_data_fail(){
        let logger = CoreLogger::new(Some("./test_log_3.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        std::fs::remove_file("./test_log_3.log").unwrap();
        global_state.get_state_data(1).unwrap();
    }

    #[test]
    fn test_update_running_pass(){
        let logger = CoreLogger::new(Some("./test_log_4.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        global_state.update_running(0).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_execution_state(), ExecutionState::Running);
        std::fs::remove_file("./test_log_4.log").unwrap()
    }

    #[test]
    #[should_panic="index out of bound"]
    fn test_update_running_fail(){
        let logger = CoreLogger::new(Some("./test_log_5.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        std::fs::remove_file("./test_log_5.log").unwrap();
        global_state.update_running(1).unwrap();
    }

    #[test]
    fn test_update_paused_pass(){
        let logger = CoreLogger::new(Some("./test_log_6.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        global_state.update_paused(0, None).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        assert_eq!(state_data.get_execution_state(), ExecutionState::Paused);
        std::fs::remove_file("./test_log_6.log").unwrap();
    }

    #[test]
    #[should_panic="index out of bound"]
    fn test_update_paused_fail(){
        let logger = CoreLogger::new(Some("./test_log_7.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        std::fs::remove_file("./test_log_7.log").unwrap();
        global_state.update_paused(1, None).unwrap();
    }

    #[test]
    fn test_update_result_pass(){
        let logger = CoreLogger::new(Some("./test_log_8.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        global_state.update_running(0).unwrap();
        let data = Value::String("some result".to_string());
        global_state.update_result(0, data.clone(), true).unwrap();
        let state_data = global_state.get_state_data(0).unwrap();
        std::fs::remove_file("./test_log_8.log").unwrap();
        assert_eq!(state_data.get_result().unwrap(), data);
    }

    #[test]
    #[should_panic="index out of bound"]
    fn test_update_result_fail(){
        let logger = CoreLogger::new(Some("./test_log_9.log"));
        let mut global_state = GlobalState::new(logger);
        global_state.new_workflow(0, "test_workflow");
        std::fs::remove_file("./test_log_9.log").unwrap();
        global_state.update_result(1, Value::Null, true).unwrap();
    }

}