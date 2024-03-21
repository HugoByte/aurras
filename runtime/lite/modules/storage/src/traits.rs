use crate::storage::CustomError;
use uuid::Uuid;

pub trait Storage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, CustomError>;
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn delete_data(&self, key: &str) -> Result<(), CustomError>;
    fn store_wasm(&self, wasm : &[u8]) -> Result<Uuid, CustomError>;
    fn get_wasm(&self, event_id: &Uuid) -> Result<Vec<u8>, CustomError>;
}
