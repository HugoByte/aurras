use super::*;
use crate::errors::AppError;
use crate::{db::UserRepository, openwhisk_model::*};
use actix_extract_multipart::Multipart;
use actix_web::web::Json;
use openwhisk_rust::{Action, Exec, KeyValue, Rule, Trigger};
use reqwest::StatusCode;
use tracing::info;

// Handle for creating an action
pub async fn action_create(
    data: Multipart<ActionInput>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;
    info!("{}", serde_json::to_value(&user).unwrap());
    match user {
        Some(u) => {
            info!("{}", serde_json::to_value(&u).unwrap());
            let res = action_create_query(data).await;
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}

// Funtion for creating an action , data comes from the handle
pub async fn action_create_query(data: Multipart<ActionInput>) -> HttpResponse {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let bas64_data = base64::encode(data.file.data());

    let action = Action {
        namespace: data.namespace.clone(),
        name: data.name.clone(),
        version: "0.0.1".to_string(),
        limits: Default::default(),
        exec: Exec {
            kind: data.kind.clone(),
            code: bas64_data,
            image: data.image.clone(),
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
    };

    let url = format!(
        "{}/api/v1/namespaces/{}/actions/{}?overwrite=true",
        data.url, data.namespace, data.name
    );
    let auth = data.auth.split(":").collect::<Vec<&str>>();

    let body = client
        .put(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .json(&action)
        .send()
        .await
        .unwrap();

    info!("{:?}", body);
    let res = format!(
        "{}/api/v1/namespaces/{}/actions/{}?blocking=true&result=true",
        data.url, data.namespace, data.name
    );

    HttpResponse::Ok().json(res)
}

// query for deleting an action, trigger or a rule.
pub async fn delete_query(data: Json<Delete>) -> HttpResponse {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let url: String;
    if data.deleting_type == "action".to_string() {
        url = format!(
            "{}/api/v1/namespaces/{}/actions/{}",
            data.url, data.namespace, data.name
        );
    } else if data.deleting_type == "trigger".to_string() {
        url = format!(
            "{}/api/v1/namespaces/{}/triggers/{}",
            data.url, data.namespace, data.name
        );
    } else {
        url = format!(
            "{}/api/v1/namespaces/{}/rules/{}",
            data.url, data.namespace, data.name
        );
    }
    let auth = data.auth.split(":").collect::<Vec<&str>>();

    let body = client
        .delete(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .send()
        .await
        .unwrap();

    info!("{:?}", body);

    let get_url = format!("{}/api/v1/namespaces/{}/actions/", data.url, data.namespace);

    let list = client
        .get(get_url)
        .basic_auth(auth[0], Some(auth[1]))
        .send()
        .await
        .unwrap();

    let res = list.text().await.unwrap();

    HttpResponse::Ok().json(res)
}

// function for creating an trigger
pub async fn trigger_create_query(data: Json<TriggerInput>) -> HttpResponse {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let param: Vec<KeyValue>;
    if data.param_json.is_empty() {
        param = Vec::new();
    } else {
        param = serde_json::from_str(&data.param_json).unwrap();
    }
    let trigger = Trigger {
        namespace: data.namespace.clone(),
        name: data.name.clone(),
        version: "0.0.1".to_string(),
        publish: Default::default(),
        updated: Default::default(),
        annotations: Default::default(),
        parameters: param.clone(),
        limits: Default::default(),
    };

    info!("{:?}", trigger);

    let url = format!(
        "{}/api/v1/namespaces/{}/triggers/{}?overwrite=true",
        data.url, data.namespace, data.name
    );
    let auth = data.auth.split(":").collect::<Vec<&str>>();

    let trigger_body = client
        .put(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .json(&trigger)
        .send()
        .await
        .unwrap();

    info!("{:?}", trigger_body);

    let url = format!(
        "{}/api/v1/namespaces/{}/rules/{}?overwrite=true",
        data.url, data.namespace, data.rule
    );
    let trigger = format!("/{}/{}/", data.namespace.clone(), data.name.clone());

    let action = format!("/{}/{}/", data.namespace.clone(), data.action.clone());

    let rule_body = serde_json::to_value(Rule {
        name: data.rule.clone(),
        trigger,
        action,
    })
    .unwrap();

    let rule_body = client
        .put(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .json(&rule_body)
        .send()
        .await
        .unwrap();

    info!("{:?}", rule_body);

    let res = format!(
        "{}/api/v1/namespaces/{}/triggers/{}",
        data.url, data.namespace, data.name
    );

    HttpResponse::Ok().json(format!("{}", res))
}

// Handle for delete
pub async fn delete(
    data: Json<Delete>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;

    match user {
        Some(u) => {
            info!("{:?} found", u.username);
            let res = delete_query(data).await;
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}

// Handle for creating a trigger
pub async fn create_trigger(
    data: Json<TriggerInput>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;

    match user {
        Some(u) => {
            info!("{:?} found", u.username);
            let res = trigger_create_query(data).await;
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}

// Handle for creating a trigger
pub async fn get_list(
    data: Json<List>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;

    match user {
        Some(u) => {
            info!("{:?} found", u.username);
            let res = get_list_query(data).await;
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}

use serde_json::Error;
// function for creating an trigger
pub async fn get_list_query(data: Json<List>) -> HttpResponse {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let url = format!(
        "{}/api/v1/namespaces/{}/{}/",
        data.url, data.namespace, data.list_type
    );
    let auth = data.auth.split(":").collect::<Vec<&str>>();

    let list = client
        .get(url)
        .basic_auth(auth[0], Some(auth[1]))
        .send()
        .await
        .unwrap();

    let actions: Result<Vec<Action>, Error> =
        serde_json::from_str(list.text().await.unwrap().as_str());
    let res = match actions {
        Ok(actions) => {
            let mut result = Vec::new();
            for action in actions.into_iter() {
                let actionlist = ActionList {
                    name: action.name,
                    namespace: action.namespace,
                };

                result.push(actionlist)
            }

            Ok(result)
        }
        Err(error) => Err(format!("Failed to deserailize actions {}", error)),
    };

    HttpResponse::Ok().json(res.unwrap())
}


#[actix_web::test]
async fn trigger_test(){
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
    assert_eq!(res.status(),StatusCode::OK);
}

#[actix_web::test]
async fn delete_rule_test(){
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
    assert_eq!(res.status(),StatusCode::OK);
}

#[actix_web::test]
async fn delete_trigger_test(){
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
    assert_eq!(res.status(),StatusCode::OK);
}

#[actix_web::test]
async fn delete_action_test(){
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
    assert_eq!(res.status(),StatusCode::OK);
}

#[actix_web::test]
async fn get_action_list_test(){
    let inp = r#"{
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "list_type": "actions"
    }"#;
    let data = serde_json::from_str::<List>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = get_list_query(input).await;
    assert_eq!(res.status(),StatusCode::OK);
}

#[actix_web::test]
async fn get_trigger_list_test(){
    let inp = r#"{
        "url": "http://localhost:3233",
        "namespace": "guest",
        "auth": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
        "list_type": "triggers"
    }"#;
    let data = serde_json::from_str::<List>(&inp);
    let input = actix_web::web::Json(data.unwrap());
    let res = get_list_query(input).await;
    assert_eq!(res.status(),StatusCode::OK);
}