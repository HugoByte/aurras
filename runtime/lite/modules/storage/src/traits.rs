pub trait Storage {
    pub fn context(&self) -> &CoreStorage
    pub fn get_data(&self, key: &str) -> Result<Vec<u8>, Error>,
    pub fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error>,
    pub fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error>,
    pub fn delete_data(&self, key: &str) -> Result<(), Error>
}

pub trait Bucket {
    fn get(&self, key: &str) -> Result<Vec<u8>, Error>,
    fn has_key(&self, key: &str) -> Result<bool, Error>,
    fn insert(&self, key:&str, value:Vec<u8>) -> Result<(), Error>,
    fn set(&mut self, key: &str, value: Vec<u8>) -> Result<(), Error>,
    fn delete(&mut self, key: &str) -> Result<(), Error>,
    fn get_and_delete(&mut self, key: &str) -> Result<Vec<u8>, Error>,
}
