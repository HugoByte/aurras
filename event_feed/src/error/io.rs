use super::*;

#[derive(Error, Debug)]
pub enum IOError {
    Anyhow(Error),
    Other(String),
    Std(Box<std::io::Error>),
    Subxt(subxt::Error),
}

impl IOError {
    pub fn into_anyhow(self) -> anyhow::Error {
        match self {
            IOError::Anyhow(error) => error,
            IOError::Other(msg) => anyhow::Error::msg(msg),
            IOError::Std(io_error) => anyhow::Error::new(*io_error),
            IOError::Subxt(subxt_error) => anyhow::Error::msg(subxt_error.to_string()),
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
