use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pub {
    pub id: String,
    pub data: Vec<Consumer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Consumer {
    pub wasm: Vec<u8>,
    pub pub_key: String,
    pub event_id : String,
    pub input_data: serde_json::Value,
}

pub fn update_with(dest: &mut serde_json::Value, src: &serde_json::Value) {
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



