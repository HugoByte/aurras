#[cfg(test)]
use crate::models::User;
#[cfg(test)]
use crate::tests::helper::*;
#[cfg(test)]
use crate::{
    handler::{openwhisk_action::*, openwhisk_rule_trigger::*},
    models::openwhisk_model::*,
};
#[cfg(test)]
use reqwest::StatusCode;
#[cfg(test)]
use uuid::Uuid;
#[cfg(test)]
use wiremock::matchers::{method, path};
#[cfg(test)]
use wiremock::{MockServer, Mock, ResponseTemplate};

#[actix_web::test]
async fn trigger_test() {
    let server = delete_uri().await;
    let trigger_input = TriggerInput{
        name: "test_trigger".to_string(),
        param_json: "[{\"key\":\"car_type\",\"value\":\"hatchback\"}]".to_string(),
        url:server.uri(),       
        namespace: "guest".to_string(),
        auth:"23b46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
    };
    let input = actix_web::web::Json(trigger_input);
    let res = trigger_create_query(input).await;
    
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_rule_test() {
    let server = delete_uri().await;
    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["test_rule".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let delete = Delete{
        name: "test_rule".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "rule".to_string(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_trigger_test() {
    let server = delete_uri().await;
    let delete = Delete{
        name: "trigger".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "trigger".to_string(),
    };
    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["trigger".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_action_test() {
    let server = delete_uri().await;
    let delete = Delete{
        name: "test_action".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "action".to_string(),
    };

    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec!["test_action".to_string()],
        trigger_and_rule: vec![],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_action_list_test() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(vec![action_data()])
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    let get_action = List{
        url: mock_server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        list_type: "actions".to_string(),
    };
    let input = actix_web::web::Json(get_action);
    let res = get_list_query(input).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_trigger_list_test() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/triggers/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(vec![trigger_data()])
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;
    let get_trigger = List{
        url: mock_server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        list_type: "triggers".to_string(),
    };
    let input = actix_web::web::Json(get_trigger);
    let res = get_list_query(input).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_rule_list_test() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/rules/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(vec![trigger_data()])
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;
    let get_trigger = List{
        url: mock_server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        list_type: "rules".to_string(),
    };
    let input = actix_web::web::Json(get_trigger);
    let res = get_list_query(input).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_rule_list_test_fail() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/rules/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(vec![trigger_data()])
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;
    let get_trigger = List{
        url: "http://127.0.0.1:72".to_string(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        list_type: "rule".to_string(),
    };
    let input = actix_web::web::Json(get_trigger);
    let res = get_list_query(input).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn delete_rule_test_fail() {
    let server = delete_uri().await;
    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["test_rule2".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let delete = Delete{
        name: "test_rule".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "rule".to_string(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn unatharsed_delete_action_test() {
    let server = delete_uri().await;
    let delete = Delete{
        name: "test_action".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "action".to_string(),
    };

    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec!["test".to_string()],
        trigger_and_rule: vec![],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn unauthorised_delete_rule_test() {
    let server = delete_uri().await;
    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["rule".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let delete = Delete{
        name: "test_rule".to_string(),
        url: server.uri(),
        namespace: "guest".to_string(),
        auth: "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        deleting_type: "rule".to_string(),
    };
    let input = actix_web::web::Json(delete);
    let res = delete_query(input, user).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn create_rule_test() {
    let server = put(None).await;
    let trigger_input = RuleInput{
        rule: "test_rule".to_string(),
        trigger: "trigger".to_string(),
        action: "test_action".to_string(),
        url:server.uri(),       
        namespace: "guest".to_string(),
        auth:"23b46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
    };
    let input = actix_web::web::Json(trigger_input);
    let res = create_rule_request(input).await;
    
    assert_eq!(res.status(), StatusCode::OK);
}