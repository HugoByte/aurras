use super::*;
use crate::errors::AppError;
use crate::models::{UpdateAction, User};
use crate::{db::UserRepository, openwhisk_model::*};
use actix_extract_multipart::Multipart;
use actix_web::web::Json;
use openwhisk_rust::{Action, Exec, KeyValue};
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
            let mut update = UpdateAction { actions: u.actions };
            let action_name = data.name.clone();
            let res = action_create_query(data).await;
            if res.status().is_success() {
                if !update.actions.contains(&action_name) {
                    update.actions.push(action_name);
                }
                repository.update_user_action(update, u.id).await?;
            }
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
            let res = delete_query(data, u).await;
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}

// query for deleting an action, trigger or a rule.
pub async fn delete_query(data: Json<Delete>, user: User) -> HttpResponse {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let url: String;
    if data.deleting_type == "action".to_string() {
        if user.actions.contains(&data.name) {
            url = format!(
                "{}/api/v1/namespaces/{}/actions/{}",
                data.url, data.namespace, data.name
            );
        } else {
            return HttpResponse::Unauthorized().into();
        }
    } else if data.deleting_type == "trigger".to_string() {
        if user.trigger_and_rule.contains(&data.name) {
            url = format!(
                "{}/api/v1/namespaces/{}/triggers/{}",
                data.url, data.namespace, data.name
            );
        } else {
            return HttpResponse::Unauthorized().into();
        }
    } else {
        if user.trigger_and_rule.contains(&data.name) {
            url = format!(
                "{}/api/v1/namespaces/{}/rules/{}",
                data.url, data.namespace, data.name
            );
        } else {
            return HttpResponse::Unauthorized().into();
        }
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
        None => Err(AppError::NOT_AUTHORIZED.into()),
    }
}

use serde_json::{Error, Value};
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

    let list = match client
        .get(url)
        .basic_auth(auth[0], Some(auth[1]))
        .send()
        .await{
            Ok(res) => res,
            Err(_) => return HttpResponse::NotFound().into(),
        };

    let actions: Result<Vec<Value>, Error> =
        serde_json::from_str(list.text().await.unwrap().as_str());
    match actions {
        Ok(actions) => {
            let mut result = Vec::new();
            for action in actions.into_iter() {
                let actionlist = ActionList {
                    name: action.get("name").unwrap().to_string(),
                    namespace: action.get("namespace").unwrap().to_string(),
                };

                result.push(actionlist)
            }

            HttpResponse::Ok().json(result)
        }
        Err(error) => HttpResponse::from_error(error),
    }
}
