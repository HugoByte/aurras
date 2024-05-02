use super::traits::Storage;
use crate::AppError;
use crate::RuntimeError;
use rocksdb::DB;
use serde::Serialize;

/// Represents a core database.
#[derive(Debug)]
pub struct CoreStorage {
    pub db: rocksdb::DB,
    cache_db: bool,
    is_refreshed: bool,
}

impl CoreStorage {
    /// A constructor function for `CoreStorage`.
    ///
    /// # Arguments:
    ///
    /// * `db_name`: Name of the database to be created.
    pub fn new(db_name: &str) -> Result<Self, RuntimeError> {
        let db = DB::open_default(db_name).map_err(RuntimeError::RocksDB)?;
        Ok(Self {
            db,
            cache_db: false,
            is_refreshed: false,
        })
    }

    /// A constructor function for `CoreStorage` with a TTL(Time To Live) configuration.
    ///
    /// # Arguments:
    ///
    /// * `db_name`: Name of the database to be created.
    /// * `ttl_duration_seconds`: Number of seconds that the data should be stored in the database.
    /// The data will be deleted after the specified time.
    pub fn new_with_ttl(
        db_name: &str,
        ttl_duration_seconds: u64,
        is_refreshed: bool,
    ) -> Result<Self, RuntimeError> {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);

        let db = DB::open_with_ttl(
            &opts,
            db_name,
            std::time::Duration::from_secs(ttl_duration_seconds),
        )
        .map_err(RuntimeError::RocksDB)?;

        Ok(Self {
            db,
            cache_db: true,
            is_refreshed,
        })
    }

    /// This function cleans the expired data from the database that is configured with ttl.
    pub fn clean_expired_data(&self) {
        if self.cache_db {
            // Trigger a manual compaction, this will check the TTL filter
            // in the database and drop all expired entries.
            self.db.compact_range(None::<&[u8]>, None::<&[u8]>);
        }
    }

    /// This function refreshes the lifetime of the data in the database that is configured with ttl.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    /// * `value`: The data to be re-stored in the database.
    fn refresh_lifetime(&self, key: &str, value: &Vec<u8>) -> Result<(), RuntimeError> {
        if self.cache_db && self.is_refreshed {
            self.db.put(key, value)?;
        }

        Ok(())
    }
}

impl Storage for CoreStorage {
    /// Retrieves data from the database using the provided key.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    ///
    /// Returns:
    ///
    /// Returns the data as bytes.
    fn get_data(&self, key: &str) -> Result<Vec<u8>, RuntimeError> {
        self.clean_expired_data();

        let datastore = self.db.get(key)?;
        let data = match datastore {
            Some(ivec) => {
                self.refresh_lifetime(key, &ivec)?;
                ivec
            }
            None => {
                return Err(RuntimeError::AppError(AppError::new(
                    "Data not found".to_string(),
                )))
            }
        };
        Ok(data)
    }

    /// Stores data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Unique identifier for the data.
    /// * `value`: The data to be stored in the database.
    fn set_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError> {
        self.clean_expired_data();
        self.db.put(key, value)?;
        Ok(())
    }

    /// Modifies data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    /// * `value`: The data to be modified in the database.
    fn modify_data(&self, key: &str, value: Vec<u8>) -> Result<(), RuntimeError> {
        self.clean_expired_data();
        let _existing_data = self.get_data(key)?;
        self.db.put(key, value)?;

        Ok(())
    }

    /// Deletes data associated with a given key from a database.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    fn delete_data(&self, key: &str) -> Result<(), RuntimeError> {
        self.clean_expired_data();
        self.db.delete(key)?;
        Ok(())
    }

    /// Retrieves the data as a generic type from the database using the provided key.
    ///
    /// # Arguments:
    ///
    /// * `key`: Key of the data in the database.
    fn get_value<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, RuntimeError> {
        self.clean_expired_data();
        let res = self
            .db
            .get(key)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        match res {
            Some(bytes) => {
                self.refresh_lifetime(key, &bytes)?;
                let value = serde_json::from_slice(&bytes)
                    .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Inserts a generic type data in the database using the provided key and value.
    ///
    /// # Arguments:
    ///
    /// * `key`: Unique identifier for the data.
    /// * `value`: The value to be stored in the database.
    fn insert_value<T: Serialize>(&self, key: &str, value: T) -> Result<(), RuntimeError> {
        self.clean_expired_data();
        let bytes = serde_json::to_vec(&value)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))?;
        self.db
            .put(key, bytes)
            .map_err(|e| RuntimeError::AppError(AppError::new(e.to_string())))
    }
}
