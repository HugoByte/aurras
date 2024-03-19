use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct Resultss {
    result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    pub allowed_hosts: Option<Vec<String>>,
    pub data: Value,
}