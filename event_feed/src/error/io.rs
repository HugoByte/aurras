use super::*;

#[derive(Error, Debug)]
pub enum IOError {
    Anyhow(Error),
    Other(String),
    Std(Box<std::io::Error>),
    Subxt(subxt::Error),
}

impl From<anyhow::Error> for IOError {
    fn from(err: anyhow::Error) -> Self {
        match err.root_cause().downcast_ref::<std::io::Error>() {
            Some(io_err) => IOError::Std(Box::new(std::io::Error::new(
                io_err.kind(),
                io_err.to_string(),
            ))),
            None => IOError::Other(format!("Unexpected error: {}", err)),
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::Anyhow(error) => write!(f, "{}", error),
            IOError::Other(error) => write!(f, "{}", error),
            IOError::Std(error) => write!(f, "{}", error),
            IOError::Subxt(error) => write!(f, "{}", error),
        }
    }
}
