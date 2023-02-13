#[cfg(test)]
use actix_web::{http::header::ContentType, test, web, App};
#[cfg(test)]
use crate::handler::index;

#[actix_web::test]
async fn test_index_get() {
    let app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_index_post() {
    let app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
