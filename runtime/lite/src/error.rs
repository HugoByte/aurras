use crate::{
    kuska_ssb_client::{self},
    storage::{self},
};

#[derive(Debug)]
pub enum CommonError {
    CustomError(storage::error::CustomError),
    AppError(kuska_ssb_client::client::AppError),
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::CustomError(err) => write!(f, "Storage Error: {}", err),
            CommonError::AppError(err) => write!(f, "Kusaka SSB Client Error: {}", err),
        }
    }
}