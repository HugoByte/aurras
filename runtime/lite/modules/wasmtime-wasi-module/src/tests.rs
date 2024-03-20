#[async_std::test]
async fn test_hello_world() {
    let path = std::env::var("WORKFLOW_WASM")
        .unwrap_or("../../../../workflow/examples/hello_world.wasm".to_string());

    let server = super::post("127.0.0.1:8080").await;
    let input = serde_json::json!({
        "allowed_hosts": [
            server.uri()
        ],
        "data": {
           "hello" : "world"
        }
    });

    let result = super::run_workflow(input, path, 1).unwrap();

    assert!(result.result.to_string().contains("Hello"));
}

#[async_std::test]
async fn test_employee_salary() {

    let path = std::env::var("WORKFLOW_WASM").unwrap_or(
        "../../../../workflow/examples/employee_salary_state_managed.wasm".to_string(),
    );

    let server = test_util::post("127.0.0.1:1234").await;
    let input = serde_json::json!({
        "allowed_hosts": [
            server.uri()
        ],
        "data": {
            "role":"Software Developer",
        }
    });

    let result = super::run_workflow(input.clone(), path.clone(), 2).unwrap();
    assert!(result
        .result
        .to_string()
        .contains("Salary creditted for emp id 1 from Hugobyte"));
}
