pub mod traits;
pub use traits::*;
pub mod storage;
pub use storage::*;

#[cfg(test)]
mod tests {
    use crate::storage::CoreStorage;
    use crate::Storage;
    pub use rocksdb::{ErrorKind, DB};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_get_data() {
        let lock_file_path = "my_db.db/LOCK";
        let mut retries = 0;

        while retries < 3 {
            if let Err(_) = std::fs::remove_file(lock_file_path) {
                println!("Failed to remove lock file: {}", lock_file_path);
                retries += 1;
                thread::sleep(Duration::from_secs(1)); // Wait for 1 second before retrying
            } else {
                break;
            }
        }

        let db = DB::open_default("my_db.db").unwrap();
        let core_storage = CoreStorage::new("test_id".to_string(), db);
        core_storage.db.put("test_key", b"test_value").unwrap();
        let result = core_storage.get_data("test_key").unwrap();
        assert_eq!(result, b"test_value");
    }

    #[test]
    fn test_set_data() {
        let lock_file_path = "my_db1.db/LOCK";
        let mut retries = 0;

        while retries < 3 {
            if let Err(_) = std::fs::remove_file(lock_file_path) {
                println!("Failed to remove lock file: {}", lock_file_path);
                retries += 1;
                thread::sleep(Duration::from_secs(1)); // Wait for 1 second before retrying
            } else {
                break;
            }
        }

        let db = DB::open_default("my_db1.db").unwrap();
        let core_storage = CoreStorage::new("test_id".to_string(), db);
        core_storage
            .set_data("test_key", b"test_value".to_vec())
            .unwrap();
        let result = core_storage.get_data("test_key").unwrap();
        println!("{:?}", result);
        let deserialized_value: Vec<u8> = rmp_serde::from_slice(&result).unwrap();
        assert_eq!(deserialized_value, b"test_value");
    }

    #[test]
    fn test_modify_data() {
        let lock_file_path = "my_db2.db/LOCK";
        let mut retries = 0;

        while retries < 3 {
            if let Err(_) = std::fs::remove_file(lock_file_path) {
                println!("Failed to remove lock file: {}", lock_file_path);
                retries += 1;
                thread::sleep(Duration::from_secs(1)); // Wait for 1 second before retrying
            } else {
                break;
            }
        }

        let core_storage = CoreStorage::new(
            "test_id".to_string(),
            DB::open_default("my_db2.db").unwrap(),
        );
        let key = "test_key";
        let value = vec![1, 2, 3];

        core_storage.set_data(key, value.clone()).unwrap();
        core_storage.modify_data(key, vec![4, 5, 6]).unwrap();

        let modified_data = core_storage.get_data(key).unwrap();
        assert_eq!(modified_data, vec![4, 5, 6]);
    }

    #[test]
    fn test_delete_data() {
        let core_storage = CoreStorage::new(
            "test_id".to_string(),
            DB::open_default("my_db3.db").unwrap(),
        );
        let key = "test_key";
        let value = vec![1, 2, 3];

        core_storage.set_data(key, value).unwrap();
        core_storage.delete_data(key).unwrap();

        let deleted_data = core_storage.get_data(key).unwrap_or_else(|e| {
            assert_eq!(e.kind(), ErrorKind::NotFound);
            Vec::new()
        });

        assert_eq!(deleted_data, Vec::new());
    }

    #[test]
    #[should_panic]
    fn test_with_different_key() {
        let core_storage = CoreStorage::new(
            "test_id".to_string(),
            DB::open_default("test_db.db").unwrap(),
        );
        let _key = "key";

        let result = core_storage.get_data("test_key");
        assert!(result.is_err());
    }
}
