use super::*;

#[derive(Error, Debug)]
pub enum IOError {
    Anyhow(Error),
    Other(String),
    Std(Box<std::io::Error>),
    Subxt(subxt::Error),
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
