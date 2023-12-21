pub trait Exception: std::fmt::Display + std::fmt::Debug {
    fn code(&self) -> i32;
}

pub trait Execute<T> {
    type Input;
    type Output;

    fn execute(self, context: T) -> Result<Self::Output, Box<dyn Exception>>;
}

pub type Result<T, E = Box<dyn Exception>> = core::result::Result<T, E>;
