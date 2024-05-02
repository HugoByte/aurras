use rocksdb::Error as RocksDBError;
use std::{io, sync::PoisonError};

use crate::Ctx;

/// Define an error type for the Lite runtime.
#[derive(Debug)]
pub struct AppError {
    message: String,
}
impl AppError {
    /// Creates a new `AppError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message`: The error message.
    pub fn new(message: String) -> Self {
        AppError { message }
    }
}

impl std::error::Error for AppError {
    /// Returns a string describing the error.
    fn description(&self) -> &str {
        &self.message
    }
}

/// Implements the `std::fmt::Display` trait for `AppError` to format the error messages.
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Define a list of errors that can occur in the Lite runtime.
#[derive(Debug)]
pub enum RuntimeError {
    AppError(AppError),
    RocksDB(RocksDBError),
    Io(io::Error),
    Serde(serde_json::Error),
}

/// Implements the `From` trait to convert the `RocksDBError` to the `RuntimeError::RocksDB` error type.
impl From<RocksDBError> for RuntimeError {
    fn from(error: RocksDBError) -> Self {
        RuntimeError::RocksDB(error)
    }
}

/// Implements the `From` trait to convert the `io::Error` to the `RuntimeError::Io` error type.
impl From<io::Error> for RuntimeError {
    fn from(error: io::Error) -> Self {
        RuntimeError::Io(error)
    }
}

/// Implements the `From` trait to convert the `serde_json::Error` to the `RuntimeError::Serde` error type.  
impl From<RuntimeError> for io::Error {
    fn from(error: RuntimeError) -> Self {
        match error {
            RuntimeError::RocksDB(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::Io(e) => e,
            RuntimeError::AppError(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::Serde(e) => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

/// Implements the `std::fmt::Display` trait to format the `RuntimeError` errors.
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::AppError(e) => write!(f, "Kusaka SSB Client Error: {}", e),
            RuntimeError::RocksDB(e) => write!(f, "Rocksdb error: {}", e),
            RuntimeError::Io(e) => write!(f, "Io error : {}", e),
            RuntimeError::Serde(error) => write!(f, "Custom error: {}", error),
        }
    }
}

/// Implements the `From` trait to convert the `PoisonError` to the `RuntimeError::AppError` error type.
impl From<PoisonError<std::sync::MutexGuard<'_, (dyn Ctx + 'static)>>> for RuntimeError {
    fn from(e: PoisonError<std::sync::MutexGuard<'_, (dyn Ctx + 'static)>>) -> Self {
        RuntimeError::AppError(AppError::new(format!("Mutex Poisoned: {:?}", e)))
    }
}

/// Implements the `From` trait to convert the `serde_json::Error` to the `RuntimeError::Serde` error type.
impl From<serde_json::Error> for RuntimeError {
    fn from(value: serde_json::Error) -> Self {
        RuntimeError::Serde(value)
    }
}
