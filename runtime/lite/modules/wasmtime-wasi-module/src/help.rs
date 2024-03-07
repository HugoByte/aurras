use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn create_server(add: &str) -> MockServer {
    let listener = std::net::TcpListener::bind(add).unwrap();
    MockServer::builder().listener(listener).start().await
}

pub async fn post(address: &str) -> MockServer {
    let server = create_server(address).await;

    let mut r = HashMap::new();
    r.insert(
        "maruthi".to_string(),
        vec!["800".to_string(), "alto".to_string()],
    );

    let res = Cartype {
        car_company_list: r,
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/namespaces/guest/actions/cartype"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(res),
        )
        .mount(&server)
        .await;
    let res = ModelAvail {
        models: vec!["800".to_string(), "alto".to_string()],
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/namespaces/guest/actions/modelavail"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(res),
        )
        .mount(&server)
        .await;

    let mut r = HashMap::new();
    r.insert("800".to_string(), 1200000);
    r.insert("alto".to_string(), 1800000);

    let res = ModelPrice {
        model_price_list: r,
    };
    Mock::given(method("POST"))
        .and(path("/api/v1/namespaces/guest/actions/modelsprice"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(res),
        )
        .mount(&server)
        .await;

    let res = Purchase {
        message: String::from("Thank you for the purchase"),
    };
    Mock::given(method("POST"))
        .and(path("/api/v1/namespaces/guest/actions/purchase"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(res),
        )
        .mount(&server)
        .await;

    server
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cartype {
    car_company_list: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelAvail {
    models: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelPrice {
    model_price_list: HashMap<String, i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Purchase {
    message: String,
}
