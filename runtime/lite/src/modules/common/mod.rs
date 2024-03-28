use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub wasm: Vec<u8>,
    pub invite: String,
    pub pub_id: String,
    pub allowed_hosts: Option<Vec<String>>,
    pub input: Value,
}

pub fn combine_values(dest: &mut serde_json::Value, src: &serde_json::Value) {
    use serde_json::Value::{Null, Object};

    match (dest, src) {
        (&mut Object(ref mut map_dest), Object(map_src)) => {
            // map_dest and map_src both are Map<String, Value>
            for (key, value) in map_src {
                // if key is not in map_dest, create a Null object
                // then only, update the value
                *map_dest.entry(key.clone()).or_insert(Null) = value.clone();
            }
        }
        (_, _) => panic!("update_with only works with two serde_json::Value::Object s"),
    }
}
