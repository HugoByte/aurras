#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::Action;
    use crate::workflow::*;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct MockWorkflow {
        wasm_path: String,
    }

    impl Workflow for MockWorkflow {
        fn get_name(&self) -> &str {
            "mock_workflow"
        }

        fn get_wasm_path(&self) -> &str {
            &self.wasm_path
        }

        fn handle_event(&self, event: &str) -> Result<String, String> {
            Ok(format!("Handling mock event: {}", event))
        }
    }

    #[test]
    fn test_store_workflow() {
        let workflow = Box::new(MockWorkflow {
            wasm_path: "mock_path".to_string(),
        });
        let result = store_workflow(workflow);
        assert!(result.is_ok());
    }

    #[test]
    fn test_polkadot_workflow_get_name() {
        let workflow = PolkadotWorkflow {
            wasm_path: String::from("wasm.wasm"),
        };
        assert_eq!(workflow.get_name(), "polkadot");
    }

    #[test]
    fn test_handle_event_openwhisk() {
        let polkadot_workflow = OpenWhiskWorkflow { wasm_path: "/Users/prathiksha/Downloads/github/aurras/workflow/examples/employee_salary_mock.wasm".to_string() };
        let workflow_map = get_workflow_map();
        workflow_map.insert("openwhisk".to_string(), Box::new(polkadot_workflow));

        let result = handle_event("openwhisk", Action::OpenWhisk);
        assert_eq!(
            result,
            Ok("Handling openwhisk event: openwhisk".to_string())
        );
    }

    #[test]
    fn test_handle_event_openwhisk_mismatch() {
        let polkadot_workflow = OpenWhiskWorkflow { wasm_path: "/Users/prathiksha/Downloads/github/aurras/workflow/examples/employee_salary_mock.wasm".to_string() };
        let workflow_map = get_workflow_map();
        workflow_map.insert("openwhisk".to_string(), Box::new(polkadot_workflow));

        let result = handle_event("openwhisk", Action::Polkadot);
        assert_eq!(
            result,
            Ok("Handling openwhisk event: openwhisk".to_string())
        );
    }
}
