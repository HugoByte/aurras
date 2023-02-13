#[cfg(test)]
use crate::models::User;
#[cfg(test)]
use crate::{
    handler::{openwhisk_action::*, openwhisk_rule_trigger::*},
    models::openwhisk_model::*,
};
#[cfg(test)]
use reqwest::StatusCode;
#[cfg(test)]
use uuid::Uuid;

#[actix_web::test]
async fn trigger_test() {
    let inp = r#"{
        "name": "test_trigger",
        "param_json": "[{\"key\":\"car_type\",\"value\":\"hatchback\"}]",
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "rule":  "test_rule",
        "action": "cartype"
    }"#;
    let data = serde_json::from_str::<TriggerInput>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = trigger_create_query(input).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_rule_test() {
    let inp = r#"{
        "name": "test_rule",
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "deleting_type": "rule"
    }"#;

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
    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input, user).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_trigger_test() {
    let inp = r#"{
        "name": "test_trigger",
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "deleting_type": "trigger"
    }"#;

    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["test_trigger".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input, user).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_action_test() {
    let inp = r#"{
        "name": "test_action",
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "deleting_type": "action"
    }"#;

    let user: User = User {
        id: Uuid::parse_str("348a4973-be7c-4d05-85ec-cd2315383564").unwrap(),
        username: "admin".to_string(),
        email: "admin".to_string(),
        password_hash: "admin".to_string(),
        full_name: "admin".to_string(),
        actions: vec![],
        trigger_and_rule: vec!["test_action".to_string()],
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input, user).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_action_list_test() {
    let inp = r#"{
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "list_type": "actions"
    }"#;
    let data = serde_json::from_str::<List>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = get_list_query(input).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_trigger_list_test() {
    let inp = r#"{
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "list_type": "triggers"
    }"#;
    let data = serde_json::from_str::<List>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = get_list_query(input).await;
    assert_eq!(res.status(), StatusCode::OK);
}
