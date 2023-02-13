use super::*;
use crate::errors::AppError;
use crate::models::UpdateTriggerAndRule;
use crate::{db::UserRepository, openwhisk_model::*};
use actix_web::web::Json;
use openwhisk_rust::{KeyValue, Trigger};
use tracing::info;

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
            let mut update = UpdateTriggerAndRule {
                trigger_and_rule: u.actions,
            };
            let action_name = data.name.clone();

            let res = trigger_create_query(data).await;

            if res.status().is_success() {
                if !update.trigger_and_rule.contains(&action_name) {
                    update.trigger_and_rule.push(action_name);
                }
                repository.update_user_triiger_and_rule(update, u.id).await?;
            }
            Ok(res)
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
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

    let res = format!(
        "{}/api/v1/namespaces/{}/triggers/{}",
        data.url, data.namespace, data.name
    );

    HttpResponse::Ok().json(format!("{}", res))
}

// handler for updateing the query in to active and inactive states.
pub async fn update_rule(
    data: Json<UpdateRule>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;
    match user {
        Some(u) => {
            info!("{:?} found", u.username);
            let res = update_rule_query(
                data.url.clone(),
                data.namespace.clone(),
                data.rule.clone(),
                data.auth.clone(),
                data.active_status.clone(),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("{:?}", res)))
        }
        None => Err(AppError::NOT_AUTHORIZED.into()),
    }
}

// update rule in active and inactive status
pub async fn update_rule_query(
    url: String,
    namespace: String,
    rule: String,
    auth: String,
    active_status: String,
) -> reqwest::Response {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let auth = auth.split(":").collect::<Vec<&str>>();
    let url = format!(
        "{}/api/v1/namespaces/{}/rules/{}?overwrite=true",
        url, namespace, rule
    );

    let rule_body = serde_json::json!( {
        "name": rule.clone(),
        "status":active_status,
        "trigger":null,
        "action":null,
    });

    let rule_response = client
        .post(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .json(&rule_body)
        .send()
        .await
        .unwrap();
    rule_response
}

pub async fn create_rule(
    data: Json<RuleInput>,
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    let user = repository.find_by_id(user.0).await?;

    match user {
        Some(u) => {
            info!("{:?} found", u.username);
            let mut update = UpdateTriggerAndRule {
                trigger_and_rule: u.actions,
            };
            let action_name = data.rule.clone();

            let res = create_rule_request(data).await;

            if res.status().is_success() {
                if !update.trigger_and_rule.contains(&action_name) {
                    update.trigger_and_rule.push(action_name);
                }
                repository.update_user_triiger_and_rule(update, u.id).await?;
                return Ok(HttpResponse::Ok().body(format!("{:?})", res)));
            }else{
                Err(AppError::INTERNAL_ERROR.message(format!("{:?}", res)))
            }
            
        }
        None => Err(AppError::INTERNAL_ERROR.default()),
    }
}


pub async fn create_rule_request(
    data: Json<RuleInput>,
) -> reqwest::Response {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let url = format!(
        "{}/api/v1/namespaces/{}/rules/{}?overwrite=true",
        data.url, data.namespace, data.rule
    );
    let auth = data.auth.split(":").collect::<Vec<&str>>();
    let trigger = format!("/{}/{}/", data.namespace.clone(), data.trigger.clone());
    let action = format!("/{}/{}/", data.namespace.clone(), data.action.clone());

    let rule_body = serde_json::json!( {
        "name": data.rule.clone(),
        "status":"active",
        "trigger":trigger,
        "action":action,
    });

    let rule_response = client
        .put(url.clone())
        .basic_auth(auth[0], Some(auth[1]))
        .json(&rule_body)
        .send()
        .await
        .unwrap();
    rule_response
}
