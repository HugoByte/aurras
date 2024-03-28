#[cfg(test)]
mod tests {

    use crate::context::Ctx;
    use crate::logger::CoreLogger;
    use crate::state_manager::{GlobalState, GlobalStateManager};
    use crate::storage::CoreStorage;
    use crate::wasmtime_wasi_module::run_workflow;

    #[async_std::test]
    async fn test_hello_world() {
        let logger = CoreLogger::new(Some("./workflow-1.log"));
        let ctx = crate::context::Context::new(
            logger.clone(),
            CoreStorage::new("workflow_db_1").unwrap(),
            GlobalState::new(logger),
        );

        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../../workflow/examples/hello_world.wasm".to_string());

        let wasm = std::fs::read(&path).unwrap();

        let input = serde_json::json!({
            "allowed_hosts": [],
            "data": {
               "hello" : "world"
            }
        });

        let mut logger = ctx.get_logger();
        let mut state_manager = ctx.get_state_manager();

        let index = state_manager.new_workflow(1, "hello_world");

        let result = run_workflow(
            &mut state_manager,
            &mut logger,
            input.clone(),
            wasm,
            index,
            true,
            None,
        )
        .unwrap();

        assert!(result.result.to_string().contains("Hello"));
    }

    #[async_std::test]
    async fn test_employee_salary() {
        let logger = CoreLogger::new(Some("./workflow-2.log"));
        let ctx = crate::context::Context::new(
            logger.clone(),
            CoreStorage::new("workflow_db_2").unwrap(),
            GlobalState::new(logger),
        );

        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../../workflow/examples/employee_salary_state_managed.wasm".to_string());
        let wasm = std::fs::read(&path).unwrap();

        let server = test_util::post("127.0.0.1:1234").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "role":"Software Developer",
            }
        });

        let mut logger = ctx.get_logger();
        let mut state_manager = ctx.get_state_manager();

        let index = state_manager.new_workflow(2, "employee_salary");

        let result = run_workflow(
            &mut state_manager,
            &mut logger,
            input.clone(),
            wasm,
            index,
            true,
            None,
        )
        .unwrap();
        assert!(result
            .result
            .to_string()
            .contains("Salary creditted for emp id 1 from Hugobyte"));
    }

    #[async_std::test]
    async fn test_pause_resume_employee_salary() {
        let logger = CoreLogger::new(Some("./workflow-3.log"));
        let ctx = crate::context::Context::new(
            logger.clone(),
            CoreStorage::new("workflow_db_3").unwrap(),
            GlobalState::new(logger),
        );

        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../../workflow/examples/employee_salary_state_paused.wasm".to_string());
        let wasm = std::fs::read(&path).unwrap();

        let server = test_util::post("127.0.0.1:1235").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "role":"Software Developer",
            }
        });

        let mut logger = ctx.get_logger();
        let mut state_manager = ctx.get_state_manager();

        let index = state_manager.new_workflow(3, "employee_salary");

        let result = run_workflow(
            &mut state_manager,
            &mut logger,
            input.clone(),
            wasm.clone(),
            index,
            true,    // ignore cache
            Some(2), // pause at task 2
        )
        .unwrap();

        let expected_result = serde_json::json!({ "Ok" : { "workflow_paused" : true }});
        assert_eq!(result.result, expected_result);

        let result = run_workflow(
            &mut state_manager,
            &mut logger,
            input.clone(),
            wasm,
            index,
            false, // don't ignore cache
            None,  // don't pause
        )
        .unwrap();

        assert!(result
            .result
            .to_string()
            .contains("Salary creditted for emp id 1 from Hugobyte"));
    }
}
