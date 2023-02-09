#[cfg(test)]
use reqwest::StatusCode;
#[cfg(test)]
use crate::{models::openwhisk_model::*, handler::{openwhisk_action::*, openwhisk_rule_trigger::*}};

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
    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input).await;
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
    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input).await;
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
    let data = serde_json::from_str::<Delete>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = delete_query(input).await;
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
