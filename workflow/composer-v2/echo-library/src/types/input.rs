use super::*;

#[derive(
    Debug, Default, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize,
)]
pub struct Input {
    pub name: String,
    pub input_type: RustType,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_depend: bool,
}
