use std::fmt::Display;

use crate::types::Result;
use aurras_primitives::Exception;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IOError {
    Test,
    PathNotFound
}

pub fn io_error(err: std::io::Error) -> Box<dyn Exception> {
    Box::new(IOError::from(err))
}

impl Exception for IOError {
    fn code(&self) -> i32 {
        todo!()
    }
}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "IOError")
    }
}
