use std::fmt::Display;


use anyhow::Error;
use composer_primitives::Exception;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IOError {
    PathNotFound,
    Anyhow(Error),
    Other(String)
}

pub fn io_error(err: std::io::Error) -> Box<dyn Exception> {
    Box::new(IOError::from(err))
}

impl Exception for IOError {
    fn code(&self) -> i32 {
        match self {
            IOError::PathNotFound => 1,
            IOError::Other(_) => 2,
            IOError::Anyhow(_) => 3,
        }
    }
}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        match value.kind(){
            std::io::ErrorKind::NotFound => IOError::PathNotFound,
            std::io::ErrorKind::PermissionDenied => todo!(),
            std::io::ErrorKind::ConnectionRefused => todo!(),
            std::io::ErrorKind::ConnectionReset => todo!(),
            std::io::ErrorKind::ConnectionAborted => todo!(),
            std::io::ErrorKind::NotConnected => todo!(),
            std::io::ErrorKind::AddrInUse => todo!(),
            std::io::ErrorKind::AddrNotAvailable => todo!(),
            std::io::ErrorKind::BrokenPipe => todo!(),
            std::io::ErrorKind::AlreadyExists => todo!(),
            std::io::ErrorKind::WouldBlock => todo!(),
            std::io::ErrorKind::InvalidInput => todo!(),
            std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => todo!(),
            std::io::ErrorKind::WriteZero => todo!(),
            std::io::ErrorKind::Interrupted => todo!(),
            std::io::ErrorKind::Unsupported => todo!(),
            std::io::ErrorKind::UnexpectedEof => todo!(),
            std::io::ErrorKind::OutOfMemory => todo!(),
            std::io::ErrorKind::Other => todo!(),
            _ => IOError::Other("An unknown IO error occurred".to_string()),
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
