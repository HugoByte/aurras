use crate::traits::Storage;
use rocksdb::{Error, DB};

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
}
