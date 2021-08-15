mod types;
pub use types::{Context, Trigger, Config};
mod mock;

#[macro_use]
extern crate derive_new;


#[cfg(feature = "mock_containers")]
pub use mock::mock_containers;