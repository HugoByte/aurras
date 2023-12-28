pub use super::*;

macro_rules! impl_starlark_values {
    ($typ: ident) => {
        starlark_simple_value!($typ);

        #[starlark_value(type = stringify!($typ) )]
        impl<'v> StarlarkValue<'v> for $typ {}

        impl Display for $typ {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

mod input;
mod rust_types;
mod task;
mod workflow;

pub use input::*;
pub use rust_types::*;
pub use task::*;
pub use workflow::*;

impl_starlark_values!(Depend);
impl_starlark_values!(Task);
impl_starlark_values!(Operation);
impl_starlark_values!(Input);
impl_starlark_values!(Workflow);
