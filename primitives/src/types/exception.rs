pub trait Exception: std::fmt::Display + std::fmt::Debug {
    fn code(&self) -> i32;
}
