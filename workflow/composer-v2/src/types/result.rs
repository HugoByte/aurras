use aurras_primitives::Exception;

pub type Result<T, E = Box<dyn Exception>> = core::result::Result<T, E>;