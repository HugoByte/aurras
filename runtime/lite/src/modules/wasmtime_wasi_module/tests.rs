#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_CACHE_LIFETIME;
    use crate::context::Ctx;
    use crate::logger::CoreLogger;
    use crate::state_manager::{GlobalState, GlobalStateManager};
    use crate::storage::CoreStorage;
    use crate::wasmtime_wasi_module::run_workflow;
    use rocksdb::{Options, DB};

    #[async_std::test]
    async fn test_hello_world() {
        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../../workflow/examples/hello_world.wasm".to_string());

        let wasm = std::fs::read(&path).unwrap();

        let input = serde_json::json!({
            "allowed_hosts": [],
            "data": {
               "hello" : "world"
            }
        });

        {
            let logger = CoreLogger::new(Some("./workflow-1.log"));

            let ctx = crate::context::Context::new(
                logger.clone(),
                CoreStorage::new("./workflow_db_1").unwrap(),
                CoreStorage::new_with_ttl("./.cache_1", DEFAULT_CACHE_LIFETIME, true).unwrap(),
                GlobalState::new(logger),
            );
            let mut logger = ctx.get_logger();
            let mut state_manager = ctx.get_state_manager();

            let cache_storage = ctx.get_cache_db();
            let index = state_manager.new_workflow("1", "hello_world");

            let result = run_workflow(
                &mut state_manager,
                &mut logger,
                &cache_storage,
                input.clone(),
                wasm,
                index,
                true,
            )
            .unwrap();

            assert!(result.result.to_string().contains("Hello"));
        }

        std::fs::remove_file("./workflow-1.log").unwrap();
        DB::destroy(&Options::default(), "./workflow_db_1").unwrap();
        DB::destroy(&Options::default(), "./.cache_1").unwrap();
    }

    #[async_std::test]
    async fn test_employee_salary() {
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

        {
            let logger = CoreLogger::new(Some("./workflow-2.log"));
            let ctx = crate::context::Context::new(
                logger.clone(),
                CoreStorage::new("./workflow_db_2").unwrap(),
                CoreStorage::new_with_ttl("./.cache_2", DEFAULT_CACHE_LIFETIME, true).unwrap(),
                GlobalState::new(logger),
            );
            let mut logger = ctx.get_logger();
            let mut state_manager = ctx.get_state_manager();
            let cache_storage = ctx.get_cache_db();
            let index = state_manager.new_workflow("2", "employee_salary");

            let result = run_workflow(
                &mut state_manager,
                &mut logger,
                &cache_storage,
                input.clone(),
                wasm,
                index,
                true,
            )
            .unwrap();
            assert!(result
                .result
                .to_string()
                .contains("Salary creditted for emp id 1 from Hugobyte"));
        }

        std::fs::remove_file("./workflow-2.log").unwrap();
        DB::destroy(&Options::default(), "./workflow_db_2").unwrap();
        DB::destroy(&Options::default(), "./.cache_2").unwrap();
    }
}
