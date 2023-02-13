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

mod tests;

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