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

    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let serialized_value = rmp_serde::to_vec(&value).unwrap();
        self.db.put(key, &serialized_value)?;
        Ok(())
    }

    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let _existing_data = self.get_data(key)?;
        let modified_data = value;
        self.db.put(key, &modified_data).unwrap();
        Ok(())
    }

    fn delete_data(&self, key: &str) -> Result<(), Error> {
        self.db.delete(key).unwrap();
        Ok(())
    }

    fn store_wasm(&self, key: &str, wasm_path: &str) -> Result<(), Error> {
        let db = DB::open_default("wasm-db.db").unwrap();

        let wasm_bytes: Vec<u8> = fs::read(wasm_path).unwrap();

        db.put(key, &wasm_bytes).unwrap();

        drop(db);

        Ok(())
    }

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
