
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
use serde_json::{{Value,Error}};
use derive_enum_from_into::{{EnumFrom,EnumTryInto}};
use workflow_macro::Flow;

use std::convert::TryInto;
use paste::*;
use common::*;
use traits::*;
use types::*;
"""
