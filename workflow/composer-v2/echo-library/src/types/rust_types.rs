use super::*;

#[derive(Debug, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize)]
pub enum RustType {
    Null,
    Int,
    Uint,
    Float,
    Boolean,
    String,
    Value,
    List(Box<RustType>),
    Tuple(Box<RustType>, Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    Struct(String),
}

impl Default for RustType {
    fn default() -> RustType {
        Self::Null
    }
}

starlark_simple_value!(RustType);

#[starlark_value(type = "RustType")]
impl<'v> StarlarkValue<'v> for RustType {}

impl Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustType::Null => write!(f, "Null"),
            RustType::Int => write!(f, "i32"),
            RustType::Uint => write!(f, "u32"),
            RustType::Float => write!(f, "f32"),
            RustType::Boolean => write!(f, "bool"),
            RustType::String => write!(f, "String"),
            RustType::Value => write!(f, "Value"),
            RustType::List(item_type) => write!(f, "Vec<{item_type}>"),
            RustType::Tuple(key_type, value_type) => write!(f, "({key_type},{value_type})"),
            RustType::HashMap(key_type, value_type) => {
                write!(f, "HashMap<{key_type},{value_type}>")
            }
            RustType::Struct(name) => write!(f, "{name}"),
        }
    }
}
