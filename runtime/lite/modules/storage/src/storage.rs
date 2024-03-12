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
use crate::traits::Storage;
use rocksdb::{Error, Options, DB};
use std::fs;

pub struct CoreStorage {
    pub id: String,
    pub db: rocksdb::DB,
}

impl CoreStorage {
    pub fn new(id: String, db: rocksdb::DB) -> Self {
        Self { id, db }
    }
}

impl Storage for CoreStorage {
    /// The `fn get_data(&self, key: &str) -> Result<Vec<u8>, Error>` function in the `CoreStorage`
    /// struct is implementing the `get_data` method defined in the `Storage` trait.
    fn get_data(&self, key: &str) -> Result<Vec<u8>, Error> {
        //create and open a database
        let _db = DB::open_default("my_database.db").unwrap();
        let datastore = self.db.get(key).unwrap();
        let data: Vec<u8> = match datastore {
            Some(ivec) => ivec.iter().map(|x| *x as u8).collect(),
            None => Vec::new(), // Handle the empty case
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
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let serialized_value = rmp_serde::to_vec(&value).unwrap();
        self.db.put(key, &serialized_value)?;
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
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let _existing_data = self.get_data(key)?;
        let modified_data = value;
        self.db.put(key, &modified_data).unwrap();
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
    fn delete_data(&self, key: &str) -> Result<(), Error> {
        self.db.delete(key).unwrap();
        Ok(())
    }

    /// The function `store_wasm` stores a WebAssembly binary file in a key-value database.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is a reference to a string that represents the key under which the
    /// WebAssembly binary will be stored in the database.
    /// * `wasm_path`: The `wasm_path` parameter in the `store_wasm` function represents the file path
    /// to the WebAssembly (Wasm) file that you want to store in the database. This function reads the
    /// contents of the Wasm file as bytes and stores them in the database with the specified key.
    ///
    /// Returns:
    ///
    /// The `store_wasm` function is returning a `Result<(), Error>`.
    fn store_wasm(&self, key: &str, wasm_path: &str) -> Result<(), Error> {
        let db = DB::open_default("wasm-db.db").unwrap();

        let wasm_bytes: Vec<u8> = fs::read(wasm_path).unwrap();

        db.put(key, &wasm_bytes).unwrap();

        drop(db);

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
    fn get_wasm(&self, key: &str) -> Result<Vec<u8>, Error> {
        let db = DB::open_default("wasm-db.db").unwrap();

        let retrieved_wasm_bytes = match db.get(key) {
            Ok(Some(value)) => value,
            Ok(None) => panic!("WASM module not found with key: {:?}", key),
            Err(err) => return Err(err),
        };

        drop(db);

        Ok(retrieved_wasm_bytes)
    }
}
