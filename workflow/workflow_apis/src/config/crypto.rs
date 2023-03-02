extern crate bcrypt;
use super::*;
use actix_web::web::block;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use color_eyre::Result;
use eyre::eyre;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CryptoService {
    pub key: Arc<String>,
    pub jwt_secret: Arc<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

impl CryptoService {
    // Hashing the password
    #[instrument(skip(self, password))]
    pub fn hash_password(&self, password: String) -> Result<String> {
        let hash = hash(password, DEFAULT_COST);
        hash.map_err(|err| eyre!("Hashing error: {}", err))
    }

    // Verify the password
    #[instrument(skip(self, password, password_hash))]
    pub async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool> {
        verify(password, password_hash).map_err(|err| eyre!("Verifying error: {}", err))
    }

    // Generate auth token
    #[instrument(skip(self))]
    pub async fn generate_jwt(&self, user_id: Uuid) -> Result<String> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let now = Utc::now() + Duration::days(1); // Expires in 1 day
            let claims = Claims {
                sub: user_id,
                exp: now.timestamp(),
            };
            encode(&headers, &claims, &encoding_key)
        })
        .await
        .unwrap()
        .map_err(|e| eyre!("Creating jwt token : {}", e))
    }

    // verify the auth token
    #[instrument(skip(self, token))]
    pub async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation)
        })
        .await
        .expect("Verifying jwt token:")
        .map_err(|err| eyre!("Verifying jwt token: {}", err))
    }
}

// Token store
#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}
