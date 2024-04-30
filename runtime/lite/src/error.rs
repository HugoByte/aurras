use rocksdb::Error as RocksDBError;
use std::{io, sync::PoisonError};

use crate::Ctx;

/// Define an error type for the Lite runtime.
///
/// # Properties
///
/// * `message`: A string describing the error.
#[derive(Debug)]
pub struct AppError {
    message: String,
}

/// Implements the `AppError` struct.
impl AppError {
    /// Creates a new `AppError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message`: A string describing the error.
    ///
    /// # Returns
    ///
    /// A new `AppError` instance.
    pub fn new(message: String) -> Self {
        AppError { message }
    }
}

/// Implements the `std::error::Error` trait for `AppError`.
impl std::error::Error for AppError {
    /// Returns a string describing the error.
    fn description(&self) -> &str {
        &self.message
    }
}

/// Implements the `std::fmt::Display` trait for `AppError`.
impl std::fmt::Display for AppError {
    /// Formats the error message and returns it as a string.
    ///
    /// # Arguments
    ///
    /// * `f`: A mutable reference to a `std::fmt::Formatter` object.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` indicating success or failure of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Define a list of errors that can occur in the Lite runtime.
///
/// # Variants
///
/// * `AppError`: Represents an application error.
/// * `RocksDB`: Represents an error in the code that used RocksDB database.
/// * `Io`: Represents an I/O error.
/// * `Serde`: Represents a serialization or deserialization error.
#[derive(Debug)]
pub enum RuntimeError {
    AppError(AppError),
    RocksDB(RocksDBError),
    Io(io::Error),
    Serde(serde_json::Error),
}

/// Implements the `From` trait of the `RocksDBError` type for the `RuntimeError` for converting
/// it to the `RuntimeError::RocksDB` error type.
impl From<RocksDBError> for RuntimeError {
    /// Converts the `RocksDBError` to the `RuntimeError::RocksDB` error type.
    ///
    /// # Arguments
    ///
    /// * `error`: A `RocksDBError` instance.
    ///
    /// # Returns
    ///
    /// A `RuntimeError::RocksDB` error type.
    fn from(error: RocksDBError) -> Self {
        RuntimeError::RocksDB(error)
    }
}

/// Implements the `From` trait of the `io::Error` type for the `RuntimeError` for converting
/// it to the `RuntimeError::Io` error type.
impl From<io::Error> for RuntimeError {
    /// Converts the `io::Error` to the `RuntimeError::Io` error type.
    ///
    /// # Arguments
    ///
    /// * `error`: A `io::Error` instance.
    ///
    /// # Returns
    ///
    /// A `RuntimeError::Io` error type.
    fn from(error: io::Error) -> Self {
        RuntimeError::Io(error)
    }
}

/// Implements the `From` trait of the `RuntimeError` type for the `io::Error` type for converting
/// it to the `io::Error` error type.
impl From<RuntimeError> for io::Error {
    /// Converts the `RuntimeError` to the `io::Error` error type.
    ///
    /// # Arguments
    ///
    /// * `error`: A `RuntimeError` instance.
    ///
    /// # Returns
    ///
    /// A `io::Error` error type.
    fn from(error: RuntimeError) -> Self {
        match error {
            RuntimeError::RocksDB(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::Io(e) => e,
            RuntimeError::AppError(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::Serde(e) => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

/// Implements the `std::fmt::Display` trait for the `RuntimeError` struct.
impl std::fmt::Display for RuntimeError {
    /// Formats the error message and returns it as a string.
    ///
    /// # Arguments
    ///
    /// * `f`: A mutable reference to a `std::fmt::Formatter` object.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` indicating success or failure of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::AppError(e) => write!(f, "Kusaka SSB Client Error: {}", e),
            RuntimeError::RocksDB(e) => write!(f, "Rocksdb error: {}", e),
            RuntimeError::Io(e) => write!(f, "Io error : {}", e),
            RuntimeError::Serde(error) => write!(f, "Custom error: {}", error),
        }
    }
}

/// Implements the `From` trait of the `PoisonError` type for the `RuntimeError` for converting
/// it to the `RuntimeError::AppError` error type.
impl From<PoisonError<std::sync::MutexGuard<'_, (dyn Ctx + 'static)>>> for RuntimeError {
    /// Converts the `PoisonError` to the `RuntimeError::AppError` error type.
    ///
    /// # Arguments
    ///
    /// * `e`: A `PoisonError` instance.
    ///
    /// # Returns
    ///
    /// A `RuntimeError::AppError` error type.
    fn from(e: PoisonError<std::sync::MutexGuard<'_, (dyn Ctx + 'static)>>) -> Self {
        RuntimeError::AppError(AppError::new(format!("Mutex Poisoned: {:?}", e)))
    }
}

/// Implements the `From` trait of the `serde_json::Error` type for the `RuntimeError` for converting
/// it to the `RuntimeError::Serde` error type.
impl From<serde_json::Error> for RuntimeError {
    /// Converts the `serde_json::Error` to the `RuntimeError::Serde` error type.
    ///
    /// # Arguments
    ///
    /// * `value`: A `serde_json::Error` instance.
    ///
    /// # Returns
    ///
    /// A `RuntimeError::Serde` error type.
    fn from(value: serde_json::Error) -> Self {
        RuntimeError::Serde(value)
    }
}
