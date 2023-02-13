use crate::diesel::RunQueryDsl;
use crate::models::{UpdateAction, UpdateTriggerAndRule};
use crate::schema::userss;
use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::user::{NewUser, User},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use diesel::QueryDsl;
use diesel::{
    r2d2::{self, ConnectionManager},
    OptionalExtension, PgConnection,
};
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::diesel::ExpressionMethods;
use futures::future::{ready, Ready};

pub struct UserRepository {
    pub pool: Arc<Pool>,
}

impl UserRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
    #[instrument(skip(self, new_user))]
    pub async fn create(&self, new_user: NewUser, hashing: &CryptoService) -> Result<User> {
        let mut user = new_user;
        user.password_hash = hashing.hash_password(user.password_hash).unwrap();
        let result = diesel::insert_into(userss::table)
            .values(user)
            .get_result(&self.pool.get().unwrap());
        Ok(result.unwrap())
    }

    #[instrument(skip(self, user_action))]
    pub async fn update_user_action(
        &self,
        user_action: UpdateAction,
        user_id: Uuid,
    ) -> Result<User> {
        let result = diesel::update(userss::table)
            .filter(userss::id.eq(user_id))
            .set(user_action)
            .get_result(&self.pool.get().unwrap());
        Ok(result.unwrap())
    }

    #[instrument(skip(self, user_action))]
    pub async fn update_user_triiger_and_rule(
        &self,
        user_action: UpdateTriggerAndRule,
        user_id: Uuid,
    ) -> Result<User> {
        let result = diesel::update(userss::table)
            .filter(userss::id.eq(user_id))
            .set(user_action)
            .get_result(&self.pool.get().unwrap());
        Ok(result.unwrap())
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let conn = self.pool.get().unwrap();
        let user = userss::table
            .filter(userss::id.eq(id))
            .first::<User>(&conn)
            .optional()?;
        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.pool.get().unwrap();
        let mut items = userss::table
            .filter(userss::username.eq(username.clone()))
            .load::<User>(&conn)?;
        let res = items.pop();
        Ok(res)
    }
}

impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<Pool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
