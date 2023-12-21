use super::*;

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub enum Operation {
    Normal,
    Concat,
    Combine,
    Map(String),
}

impl Operation {
    pub fn is_map(&self) -> bool {
        match self {
            Self::Map(_) => true,
            _ => false,
        }
    }

    pub fn is_combine(&self) -> bool {
        match self {
            Self::Combine => true,
            _ => false,
        }
    }
}

impl Default for Operation {
    fn default() -> Operation {
        Self::Normal
    }
}

#[derive(Debug, Default, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize)]
pub struct Depend {
    pub task_name: String,
    pub cur_field: String,
    pub prev_field: String,
}

#[derive(
    Debug, Default, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize,
)]
pub struct Task {
    pub kind: String,
    pub action_name: String,
    pub input_arguments: Vec<Input>,
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub operation: Operation,
    pub depend_on: Vec<Depend>,
}
