use crate::storage::CustomError;

pub trait Storage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, CustomError>;
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), CustomError>;
    fn delete_data(&self, key: &str) -> Result<(), CustomError>;
    fn store_wasm(&self, key: &str, wasm_path : &str) -> Result<(), CustomError>;
    fn get_wasm(&self, key: &str) -> Result<Vec<u8>, CustomError>;
}
