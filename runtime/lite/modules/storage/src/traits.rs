use rocksdb::Error;

pub trait Storage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, Error>;
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error>;
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error>;
    fn delete_data(&self, key: &str) -> Result<(), Error>;
}
