use rocksdb::Error as RocksDBError;
use std::{
    fmt::{self, write},
    io,
};

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(message: String) -> Self {
        AppError { message }
    }
}

impl std::error::Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    AppError(AppError),
    RocksDB(RocksDBError),
    Io(io::Error),
    Custom(String),
}

impl From<RocksDBError> for RuntimeError {
    fn from(error: RocksDBError) -> Self {
        RuntimeError::RocksDB(error)
    }
}

impl From<io::Error> for RuntimeError {
    fn from(error: io::Error) -> Self {
        RuntimeError::Io(error)
    }
}

impl From<RuntimeError> for io::Error {
    fn from(error: RuntimeError) -> Self {
        match error {
            RuntimeError::RocksDB(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::Io(e) => e,
            RuntimeError::Custom(e) => io::Error::new(io::ErrorKind::Other, e),
            RuntimeError::AppError(e) => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::AppError(e) => write!(f, "Kusaka SSB Client Error: {}", e),
            RuntimeError::RocksDB(e) => write!(f, "Rocksdb error: {}", e),
            RuntimeError::Io(e) => write!(f, "Io error : {}", e),
            RuntimeError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}
