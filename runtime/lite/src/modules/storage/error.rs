
use rocksdb::Error as RocksDBError;
use std::{fmt, io};

#[derive(Debug)]
pub enum CustomError {
    RocksDB(RocksDBError),
    Io(io::Error),
    Custom(String),
}

impl From<RocksDBError> for CustomError {
    fn from(error: RocksDBError) -> Self {
        CustomError::RocksDB(error)
    }
}

impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::Io(error)
    }
}

impl From<CustomError> for io::Error {
    fn from(error: CustomError) -> Self {
        match error {
            CustomError::RocksDB(e) => io::Error::new(io::ErrorKind::Other, e),
            CustomError::Io(e) => e,
            CustomError::Custom(e) => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::RocksDB(e) => write!(f, "RocksDBError: {}", e),
            CustomError::Io(e) => write!(f, "IoError: {}", e),
            CustomError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}