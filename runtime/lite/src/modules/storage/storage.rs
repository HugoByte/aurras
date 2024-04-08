/// The `CoreStorage` struct implements the `Storage` trait for interacting with a RocksDB database,
/// providing methods for data retrieval, storage, modification, deletion, and storing/retrieving
/// WebAssembly modules.
///
/// Properties:
///
/// * `id`: The `id` property in the `CoreStorage` struct represents the identifier associated with the
/// storage instance. It is a unique identifier that can be used to distinguish one storage instance
/// from another.
/// * `db`: The `db` property in the `CoreStorage` struct is an instance of the `rocksdb::DB` type,
/// which represents a connection to a RocksDB database. This property is used to interact with the
/// underlying database for storing and retrieving data as defined in the `Storage` trait implementation
use super::traits::Storage;
use crate::AppError;
use crate::RuntimeError;
use rocksdb::DB;
use serde::Serialize;

#[derive(Debug)]
pub struct CoreStorage {
    pub db: rocksdb::DB,
}

impl CoreStorage {
    pub fn new(db_name: &str) -> Result<Self, RuntimeError> {
        let db = DB::open_default(db_name).map_err(RuntimeError::RocksDB)?;
        Ok(Self { db })
    }
}

impl Storage for CoreStorage {
    /// The `fn get_data(&self, key: &str) -> Result<Vec<u8>, Error>` function in the `CoreStorage`
    /// struct is implementing the `get_data` method defined in the `Storage` trait.
    fn get_data(&self, key: &str) -> Result<Vec<u8>, RuntimeError> {
        let datastore = self.db.get(key)?;
        let data = match datastore {
            Some(ivec) => ivec.to_vec(),
            None => {
                return Err(RuntimeError::AppError(AppError::new(
                    "Data not found".to_string(),
                )))
            }
        };
        Ok(data)
    }

    /// The function `set_data` serializes a vector of unsigned bytes and stores it in a database with a
    /// given key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is a reference to a string that serves as the identifier for the
    /// data being stored in the database. It is used to uniquely identify the data when retrieving or
    /// updating it.
    /// * `value`: The `value` parameter in the `set_data` function is of type `Vec<u8>`, which is a
    /// vector of unsigned 8-bit integers. It is the data that you want to store in the database under
    /// the specified `key`.
    ///
    /// Returns:
    ///
    /// The `set_data` function returns a `Result<(), Error>`.
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError> {
        self.db.put(key, value)?;
        Ok(())
    }

    /// The function `modify_data` takes a key and a value, retrieves existing data based on the key,
    /// updates the data with the new value, and stores it in a database.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is a reference to a string that is used to identify the data in the
    /// database. It is used to retrieve and modify the data associated with that key.
    /// * `value`: The `value` parameter in the `modify_data` function is of type `Vec<u8>`, which
    /// represents a vector of unsigned 8-bit integers. This parameter is used to update the data
    /// associated with the given `key` in the database.
    ///
    /// Returns:
    ///
    /// The `modify_data` function is returning a `Result<(), Error>`.
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError> {
        let _existing_data = self.get_data(key)?;
        self.db.put(key, value)?;

        Ok(())
    }

    /// The `delete_data` function in Rust deletes data associated with a given key from a database.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter in the `delete_data` function is a reference to a string (`&str`)
    /// that represents the key of the data you want to delete from the database.
    ///
    /// Returns:
    ///
    /// The `delete_data` function is returning a `Result<(), Error>`. This means that it returns a
    /// `Result` enum where the success case contains an empty tuple `()` and the error case contains an
    /// `Error`.
    fn delete_data(&self, key: &str) -> Result<(), RuntimeError> {
        self.db.delete(key)?;
        Ok(())
    }

    /// The function `get_wasm` retrieves a WebAssembly module from a database using a given key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter in the `get_wasm` function is a reference to a string (`&str`) that
    /// is used to look up a WebAssembly (WASM) module in a database.
    ///
    /// Returns:
    ///
    /// The `get_wasm` function returns a `Result` containing either a vector of `u8` bytes or an
    /// `Error`.

    fn insert_request_body(
        &self,
        key: &str,
        value: crate::common::RequestBody,
    ) -> Result<(), RuntimeError> {
        let bytes = serde_json::to_vec(&value)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        self.db
            .put(key, bytes)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))
    }

    fn get_request_body(&self, key: &str) -> Result<crate::common::RequestBody, RuntimeError> {
        let res = self
            .db
            .get(key)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        match res {
            Some(bytes) => {
                let value = serde_json::from_slice(&bytes)
                    .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
                Ok(value)
            }
            None => Err(RuntimeError::AppError(AppError::new(
                "Data not found".to_string(),
            ))),
        }
    }

/// The above Rust code defines functions for getting and inserting values into a database using serde
/// for serialization and deserialization.
/// 
/// Arguments:
/// 
/// * `key`: The `key` parameter in the provided code snippets is a reference to a string (`&str`). It
/// is used as a unique identifier to retrieve or insert values into a database.
/// 
/// Returns:
/// 
/// The `get_value` function returns a `Result` containing an `Option` of the deserialized value of type
/// `T`. The `insert_value` function returns a `Result` indicating success or an error of type
/// `RuntimeError`.
    fn get_value<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, RuntimeError> {
        let res = self
            .db
            .get(key)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        match res {
            Some(bytes) => {
                let value = serde_json::from_slice(&bytes)
                    .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// The function `insert_value` serializes a value to JSON and stores it in a database with a
    /// specified key.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter in the `insert_value` function is a reference to a string (`&str`).
    /// It is used as the key to store the serialized value in the database.
    /// * `value`: The `value` parameter in the `insert_value` function is of type `T`, which is
    /// required to implement the `Serialize` trait. This means that the value must be serializable to
    /// JSON using serde.
    /// 
    /// Returns:
    /// 
    /// The `insert_value` function is returning a `Result<(), RuntimeError>`.
    fn insert_value<T: Serialize>(&self, key: &str, value: T) -> Result<(), RuntimeError> {
        let bytes = serde_json::to_vec(&value)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        self.db
            .put(key, bytes)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))
    }
}
