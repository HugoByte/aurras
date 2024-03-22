use super::*;

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
