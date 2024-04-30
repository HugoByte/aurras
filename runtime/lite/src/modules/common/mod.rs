pub mod constants;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a request body with various fields including wasm
/// bytecode, invite string, public ID, allowed hosts, input value, workflow ID, and workflow name.
/// 
/// Properties:
/// 
/// * `wasm`: The `wasm` property in the `RequestBody` struct is a vector of unsigned 8-bit integers
/// (`Vec<u8>`). This property likely represents WebAssembly code that will be included in the request
/// body.
/// * `invite`: The `invite` property in the `RequestBody` struct is of type `String`.
/// * `pub_id`: The `pub_id` property in the `RequestBody` struct represents a unique identifier for the
/// request body. It is of type `String` in the struct definition.
/// * `allowed_hosts`: The `allowed_hosts` property in the `RequestBody` struct is an optional field
/// that contains a vector of strings. This field is used to specify a list of hosts that are allowed to
/// interact with the request. If this field is present, only requests coming from the specified hosts
/// will be accepted. If
/// * `input`: The `input` property in the `RequestBody` struct is of type `Value`. This means that it
/// can hold any type of JSON value, such as a string, number, boolean, array, object, or null. The
/// `Value` type is typically used in Rust serde library to represent dynamic
/// * `workflow_id`: The `workflow_id` property in the `RequestBody` struct represents the unique
/// identifier associated with a specific workflow. It is used to identify and differentiate between
/// different workflows within the system.
/// * `workflow_name`: The `workflow_name` property in the `RequestBody` struct represents the name of a
/// workflow. It is a string type field that holds the name of the workflow associated with the request
/// body.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RequestBody {
    pub wasm: Vec<u8>,
    pub invite: String,
    pub pub_id: String,
    pub allowed_hosts: Option<Vec<String>>,
    pub input: Value,
    pub workflow_id : String,
    pub workflow_name : String
}

/// The function `combine_values` merges two JSON objects, updating values from the source object to the
/// destination object.
/// 
/// Arguments:
/// 
/// * `dest`: A mutable reference to a `serde_json::Value` object. This object is expected to be an Object 
/// type, which represents a JSON object (key-value pairs). The function `combine_values` is designed to 
/// update this object by merging it with another JSON object provided in(`src`).
/// * `src`: A reference to a `serde_json::Value` object. This object represents a JSON value and can be of
/// various types such as Null, Bool, Number, String, Array, or Object.
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
