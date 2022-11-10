
cargo_dependencies = f"""
[lib]
crate-type = ["cdylib"]

[dependencies]
serde = "1.0.137"
serde_json = "1.0.81"
serde_derive = "1.0.81"
derive-enum-from-into = "0.1.1"
openwhisk-rust = "0.1.2"
openwhisk_macro = "0.1.6"
paste = "1.0.7"
dyn-clone = "1.0.7"
workflow_macro = "0.0.2"

"""
common_rs_file = f"""
use super::*;
#[derive(Debug,Flow)]
pub struct WorkflowGraph {{
    edges: Vec<(usize, usize)>,
    nodes: Vec<Box<dyn Execute>>,
}}

impl WorkflowGraph {{
    pub fn new(size: usize) -> Self {{
        WorkflowGraph {{
            nodes: Vec::with_capacity(size),
            edges: Vec::new(),
        }}
    }}
}}

#[macro_export]
macro_rules! impl_execute_trait {{
    ($ ($struct : ty), *) => {{
        
            paste!{{
                $( impl Execute for $struct {{
                    fn execute(&mut self) -> Result<(),String>{{
        self.run()
    }}

    fn get_task_output(&self) -> Types {{
        self.output().clone().into()
    }}

    fn set_output_to_task(&mut self, input: Types) {{
        self.setter(input)
    }}
                }}
            )*
        }}
    }};
}}

#[allow(dead_code, unused)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {{
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {{
        for (s_key, s_value) in &second {{
            if key.clone() == s_key.to_owned() {{
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }}
        }}
    }}
    data
}}

"""
traits_file = f"""
use super::*;

pub trait Execute : Debug + DynClone  {{
    fn execute(&mut self)-> Result<(),String>;
    fn get_task_output(&self)->Types;
    fn set_output_to_task(&mut self, inp: Types);
}}

clone_trait_object!(Execute);
"""

global_imports = f"""
mod common;
mod traits;
mod types;
use dyn_clone::{{clone_trait_object, DynClone}};
use openwhisk_macro::OpenWhisk;
use openwhisk_rust::*;
use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;
use std::fmt::Debug;
use serde_json::{{Value}};
use derive_enum_from_into::{{EnumFrom,EnumTryInto}};
use workflow_macro::Flow;

use std::convert::TryInto;
use paste::*;
use common::*;
use traits::*;
use types::*;
extern crate alloc;
use core::alloc::Layout;
"""

run_function = f"""
#[no_mangle]
pub fn _start(ptr: *mut u8, length: i32){{
    let result: Value;
    unsafe {{
        let mut vect = Vec::new();
        for i in 1..=length {{
            if let Some(val_back) = ptr.as_ref() {{
                vect.push(val_back.clone());
            }}
            *ptr = *ptr.add(i as usize);
        }}
         result  = serde_json::from_slice(&vect).unwrap();
    }}

    let res = main(data);
    let output = Output {{
        result: serde_json::to_value(res).unwrap(),
    }};
    let serialized = serde_json::to_vec(&output).unwrap();
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    std::mem::forget(ptr);
    unsafe {{
        set_output(ptr as i32, size);
    }}

}}

"""

# match main(result) {{
#         Ok(value) => {{
#             let mut data = serde_json::to_vec(&value).unwrap();

#             let len = data.len() as u64;
#             let len_slice = len.to_be_bytes().to_vec();
#             for i in 0..len_slice.len() {{
#                 data.insert(i, len_slice[i])
#             }}

#             let datas: &[u8] = &data;

#             let ptr = datas.as_ptr();

#             ptr as i32
#         }}
#         Err(err) => {{
#             let mut data = serde_json::to_vec(&err.to_string()).unwrap();

#             let len = data.len() as u64;
#             let len_slice = len.to_be_bytes().to_vec();
#             for i in 0..len_slice.len() {{
#                 data.insert(i, len_slice[i])
#             }}

#             let datas: &[u8] = &data;

#             let ptr = datas.as_ptr();
#             ptr as i32
#         }}
#     }}