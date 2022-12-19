use super::Exception;
use std::any::Any;

pub trait Execute<T> {
    type Input;
    type Output;

    fn execute(self, context: T)-> Result<Self::Output, Box<dyn Exception>>;
}