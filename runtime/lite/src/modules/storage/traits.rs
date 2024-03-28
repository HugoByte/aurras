use crate::{common::RequestBody, modules::storage::CustomError};

pub trait Storage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, CustomError>;
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn delete_data(&self, key: &str) -> Result<(), CustomError>;
    fn insert_request_body(&self, key: &str, value: RequestBody) -> Result<(), CustomError>;
    fn get_request_body(&self, key: &str) -> Result<RequestBody, CustomError>;
}
