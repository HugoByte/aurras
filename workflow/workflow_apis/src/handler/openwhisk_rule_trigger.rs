use super::*;
use crate::errors::AppError;
use crate::models::NewActionDetails;
use crate::{db::UserRepository, openwhisk_model::*};
use actix_web::web::Json;
use openwhisk_rust::{KeyValue, Rule, Trigger};
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
            let rule = NewActionDetails {
                rule: data.rule.clone(),
                action: data.action.clone(),
                trigger: data.name.clone(),
                active_status: true,
                url: data.url.clone(),
                auth: data.auth.clone(),
                namespace: data.namespace.clone(),
                user_id: u.id,
            };

            let res = trigger_create_query(data).await;

            if res.status().is_success() {
                match repository.create_rule_table(rule).await {
                    Ok(_) => Ok(res),
                    Err(err) => Err(AppError::from(err)),
                }
            } else {
                Ok(res)
            }
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
