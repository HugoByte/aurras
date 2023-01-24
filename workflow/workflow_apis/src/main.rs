#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

mod config;
use crate::config::*;
mod handler;
use handler::*;
mod models;
use models::*;
mod db;
mod errors;
mod schema;
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server");
    let pool = config.db_pool().expect("Database Configuration");
    let crypto_service = config.crypto_service();

    info!("Server run at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto_service.clone()))
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::post().to(index))).await;
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
}