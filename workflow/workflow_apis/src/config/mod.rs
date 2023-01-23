pub(crate) mod crypto;
use color_eyre::Result;
use crypto::*;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Mai config file, it will store the environment values
#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
}

impl Config {
    // Reading the environment variables.
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Configuration");
        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;

        c.try_into()
            .context("Loading configuration from environmet")
    }

    // Creating a db connection
    pub fn db_pool(&self) -> Result<Pool> {
        info!("Creating database connection pool.");
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.clone());
        Pool::builder()
            .connection_timeout(Duration::from_secs(30))
            .build(manager)
            .context("created database connection pool")
    }

    // initialize the crypto service like hashing.
    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}
