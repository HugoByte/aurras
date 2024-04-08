use serde::{de::DeserializeOwned, Serialize};

use crate::{common::RequestBody, RuntimeError};

pub trait Storage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, RuntimeError>;
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError>;
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError>;
    fn delete_data(&self, key: &str) -> Result<(), RuntimeError>;
    fn insert_request_body(&self, key: &str, value: RequestBody) -> Result<(), RuntimeError>;
    fn get_request_body(&self, key: &str) -> Result<Vec<RequestBody>, RuntimeError>;
    fn get_value<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RuntimeError>;
    fn insert_value<T: Serialize>(&self, key: &str, value: T) -> Result<(), RuntimeError>;
}
