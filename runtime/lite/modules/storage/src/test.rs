#[cfg(test)]
mod tests {
    use crate::storage::CoreStorage;
    use crate::Storage;
    pub use rocksdb::DB;
    use std::thread;
    use std::time::Duration;

    /// The test function `test_get_data` removes a lock file, opens a database, stores a key-value pair,
    /// retrieves the value, and asserts the equality of the retrieved value.
    #[test]
    fn test_get_data() {
        let lock_file_path = "my_db.db/LOCK";
        let mut retries = 0;

        while retries < 3 {
            if let Err(_) = std::fs::remove_file(lock_file_path) {
                println!("Failed to remove lock file: {}", lock_file_path);
                retries += 1;

                // Wait for 1 second before retrying
                thread::sleep(Duration::from_secs(1));
            } else {
                break;
            }
        }

        let core_storage = CoreStorage::new().unwrap();
        core_storage.db.put("test_key", b"test_value").unwrap();
        let result = core_storage.get_data("test_key").unwrap();
        assert_eq!(result, b"test_value");
    }

    /// The test_set_data function removes a lock file, opens a database, sets and retrieves data, and
    /// performs assertions in Rust.
    #[test]
    fn test_set_data() {
        let lock_file_path = "my_db1.db/LOCK";
        let mut retries = 0;

        while retries < 3 {
            if let Err(_) = std::fs::remove_file(lock_file_path) {
                println!("Failed to remove lock file: {}", lock_file_path);
                retries += 1;

                // Wait for 1 second before retrying
                thread::sleep(Duration::from_secs(1));
            } else {
                break;
            }
        }

        let core_storage = CoreStorage::new().unwrap();
        core_storage
            .set_data("test_key", b"test_value".to_vec())
            .unwrap();
        let result = core_storage.get_data("test_key").unwrap();
        println!("{:?}", result);
        let deserialized_value: Vec<u8> = rmp_serde::from_slice(&result).unwrap();
        assert_eq!(deserialized_value, b"test_value");
    }

    /// The test_modify_data function in Rust removes a lock file, creates a CoreStorage instance, sets
    /// and modifies data, and asserts the modified data.
    #[test]
    fn test_modify_data() {
        let core_storage = CoreStorage::new().unwrap();
        let key = "test_key";
        let initial_value = vec![1, 2, 3];
        core_storage.set_data(key, initial_value.clone()).unwrap();

        let new_value = vec![4, 5, 6];
        let result = core_storage.modify_data(key, new_value.clone());

        assert!(result.is_ok());

        let retrieved_data = core_storage.get_data(key).unwrap();
        assert_eq!(retrieved_data, new_value);
    }

    /// The test function `test_delete_data` in Rust creates a CoreStorage instance, sets data with a
    /// key and value, deletes the data, and then checks that the data was successfully deleted.
    #[test]
    fn test_delete_data() {
        let core_storage = CoreStorage::new().unwrap();

        // Insert a dummy key-value pair for testing
        let key = "test_key";
        let value = vec![1, 2, 3];
        core_storage.set_data(key, value).unwrap();

        let result = core_storage.delete_data(key);

        assert!(result.is_ok());
    }

    /// The test_with_different_key function tests that an error is returned when trying to retrieve data
    /// with a different key than the one stored in the CoreStorage.
    #[test]
    #[should_panic]
    fn test_with_different_key() {
        let core_storage = CoreStorage::new().unwrap();
        let _key = "key";

        let result = core_storage.get_data("test_key");
        assert!(result.is_err());
    }

    /// The test function stores a WebAssembly file in a database and then retrieves it to compare with
    /// the original file.
    #[test]
    fn test_store_and_get_wasm() {
        let core_storage = CoreStorage::new().unwrap();
        let wasm_bytes = vec![0x00, 0x61, 0x01];

        let key = "boilerplate";
        core_storage.store_wasm(key, &wasm_bytes).unwrap();

        let retrieved_wasm = core_storage.get_wasm(key).unwrap();

        assert_eq!(retrieved_wasm, wasm_bytes)
    }

    /// The test function is checking if an error is raised when trying to retrieve a WebAssembly module
    /// with a different key than the one it was stored with.
    #[test]
    #[should_panic]
    fn test_get_wasm_with_different_key() {
        let core_storage = CoreStorage::new().unwrap();
        let wasm_bytes = vec![0x00, 0x61, 0x01];

        let key = "boilerplate";
        core_storage.store_wasm(key, &wasm_bytes).unwrap();

        core_storage.get_wasm("hello").unwrap();
    }
}
