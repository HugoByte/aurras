#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::*;
    use crate::workflow::Action;

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
        let workflow = Box::new(MockWorkflow { wasm_path: "mock_path".to_string() });
        let result = store_workflow(workflow);
        assert!(result.is_ok());
    }

}
