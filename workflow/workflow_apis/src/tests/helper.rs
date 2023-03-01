use openwhisk_rust::{Action, Exec, KeyValue, RuleResponse, Trigger, Limits};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

pub fn action_data() -> Action {
    Action {
        namespace: "guest".to_string(),
        name: "test_action".to_string(),
        version: "0.0.1".to_string(),
        limits: Limits {
            timeout: 1,
            memory: 2,
            logsize: 3,
            concurrency: 3,
        },
        exec: Exec {
            kind: "rust:1.34".to_string(),
            code: "bas64_data".to_string(),
            image: "openwhisk/action-rust-v1.34".to_string(),
            init: "".to_string(),
            main: "".to_string(),
            components: vec![],
            binary: true,
        },
        error: "".to_string(),
        publish: true,
        updated: 0,
        annotations: vec![KeyValue {
            key: "feed".to_string(),
            value: serde_json::json!({}),
        }],
    }
}

fn rule_data() -> RuleResponse {
    RuleResponse {
        namespace: "guest".to_string(),
        name: "test_rule".to_string(),
        ..Default::default()
    }
}

pub fn trigger_data() -> Trigger {
    Trigger {
        namespace: "guest".to_string(),
        name: "trigger".to_string(),
        ..Default::default()
    }
}

async fn create_server() -> MockServer {
    MockServer::start().await
}

pub async fn get() -> MockServer {
    let server = create_server().await;

    let r = vec![action_data()];

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(r),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(vec!["guest"]),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/test_action"))
        .and(query_param("code", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(action_data().clone()),
        )
        .mount(&server)
        .await;

    let rules = vec![rule_data()];
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/rules/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!(rules)),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/rules/test_rule"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(rule_data().clone()),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/triggers/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(vec![trigger_data().clone()]),
        )
        .mount(&server)
        .await;

    server
}

pub async fn delete_uri() -> MockServer {
    let server = create_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/namespaces/guest/actions/test_action"))
        .and(query_param("code", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(action_data().clone()),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!([])),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/namespaces/guest/triggers/trigger"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!(trigger_data().clone())),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/triggers"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!([])),
        )
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/namespaces/guest/rules/test_rule"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!(rule_data().clone())),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/rules/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(json!([])),
        )
        .mount(&server)
        .await;
    server
}

pub async fn put(action: Option<Action>) -> MockServer {
    let server = create_server().await;

    if let Some(action) = action {
        Mock::given(method("PUT"))
            .and(path("/api/v1/namespaces/guest/actions/test_action"))
            .and(query_param("overwrite", "true"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("Content-Type", "application/json")
                    .set_body_json(action.clone()),
            )
            .mount(&server)
            .await;
    }

    let r = vec![action_data()];

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(r),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/namespaces/guest/rules/test_rule"))
        .and(query_param("overwrite", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(rule_data().clone()),
        )
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/namespaces/guest/triggers/trigger"))
        .and(query_param("overwrite", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(trigger_data().clone()),
        )
        .mount(&server)
        .await;
    server
}