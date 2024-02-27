use sled::{Db, Error};

struct CoreStorage {
    id: String,
    db: sled::Db,
}

impl CoreStorage {
    fn new(id: String, db: sled::Db) -> Self {
        Self { id, db }
    }
}

impl Storage for CoreStorage {
    fn get_data(&self, key: &str) -> Result<Vec<u8>, Error> {
        //create and open a database
        let db = Db::open("my_database.db")?;
        let datastore = self.db.get(key)?;
        let data: Vec<u8> = match datastore {
            Some(ivec) => ivec.iter().map(|x| *x as u8).collect(),
            None => Vec::new(), // Handle the empty case
        };

        Ok(data)
    }

    fn set_data(&mut self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let serialized_value = rmp_serde::to_vec(&value).unwrap();
        self.db.insert(key, serialized_value)?;
        Ok(())
    }

    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    fn delete_data(&self, key: &str) -> Result<(), Error> {
        // TODO
        Ok(())
    }
}
