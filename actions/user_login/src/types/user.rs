use serde_derive::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[allow(dead_code)]
impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name,
            email,
            password,
        }
    }
    pub fn get_password(&self) -> &String {
        &self.password
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    pub user_id: String,
}

#[allow(dead_code)]
impl UserId {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}
