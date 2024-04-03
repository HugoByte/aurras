use super::*;
#[test]
fn test_new_workflow() {
    let logger = CoreLogger::new(Some("./test_log_1.log"));
    let mut global_state = GlobalState::new(logger);
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
#[should_panic = "index out of bound"]
fn test_get_state_data_fail() {
    let logger = CoreLogger::new(Some("./test_log_3.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    std::fs::remove_file("./test_log_3.log").unwrap();
    global_state.get_state_data(1).unwrap();
}

#[test]
fn test_update_running_pass() {
    let logger = CoreLogger::new(Some("./test_log_4.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    global_state.update_running(0).unwrap();
    let state_data = global_state.get_state_data(0).unwrap();
    assert_eq!(state_data.get_execution_state(), ExecutionState::Running);
    std::fs::remove_file("./test_log_4.log").unwrap()
}

#[test]
#[should_panic = "index out of bound"]
fn test_update_running_fail() {
    let logger = CoreLogger::new(Some("./test_log_5.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    std::fs::remove_file("./test_log_5.log").unwrap();
    global_state.update_running(1).unwrap();
}

#[test]
fn test_update_paused_pass() {
    let logger = CoreLogger::new(Some("./test_log_6.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    global_state.update_paused(0, None).unwrap();
    let state_data = global_state.get_state_data(0).unwrap();
    assert_eq!(state_data.get_execution_state(), ExecutionState::Paused);
    std::fs::remove_file("./test_log_6.log").unwrap();
}

#[test]
#[should_panic = "index out of bound"]
fn test_update_paused_fail() {
    let logger = CoreLogger::new(Some("./test_log_7.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    std::fs::remove_file("./test_log_7.log").unwrap();
    global_state.update_paused(1, None).unwrap();
}

#[test]
fn test_update_result_pass() {
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
#[should_panic = "index out of bound"]
fn test_update_result_fail() {
    let logger = CoreLogger::new(Some("./test_log_9.log"));
    let mut global_state = GlobalState::new(logger);
    global_state.new_workflow(0, "test_workflow");
    std::fs::remove_file("./test_log_9.log").unwrap();
    global_state.update_result(1, Value::Null, true).unwrap();
}
