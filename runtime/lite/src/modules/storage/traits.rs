use crate::RuntimeError;
use serde::{de::DeserializeOwned, Serialize};

pub trait Storage {
    /// Retrieves data from the database using the provided key.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    ///
    /// Returns:
    ///
    /// Returns the data as bytes.
    fn get_data(&self, key: &str) -> Result<Vec<u8>, RuntimeError>;

    /// Stores data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Unique identifier for the data.
    /// * `value`: The data to be stored in the database.
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError>;

    /// Modifies data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    /// * `value`: The data to be modified in the database.
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError>;

    /// Deletes data associated with a given key from a database.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    fn delete_data(&self, key: &str) -> Result<(), RuntimeError>;

    /// Retrieves the data as a generic type from the database using the provided key.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    fn get_value<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RuntimeError>;

    /// Inserts a generic type data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Unique identifier for the data.
    /// * `value`: The value to be stored in the database.
    fn insert_value<T: Serialize>(&self, key: &str, value: T) -> Result<(), RuntimeError>;
}
