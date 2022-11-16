

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

    let res = main(result);
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

def cargo_generator(task_list):
    cargo_dependencies = ""
    for task in task_list:
        kind = task['kind']
    if kind == "OpenWhisk":
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
    else:
        cargo_dependencies = f"""
[lib]
crate-type = ["cdylib"]

[dependencies]
derive-enum-from-into = "0.1.1"
paste = "1.0.7"
dyn-clone = "1.0.7"
workflow_macro = "0.0.2"
substrate-api-client = {{ git = "https://github.com/shanithkk/substrate-api-client.git", branch = "testing_call", default-features = false, features = ["disable_target_static_assertions", "staking-xt"]}}
codec = {{ package = "parity-scale-codec", features = ["derive"], version = "3.0.0" }}
sp-core = {{ version = "6.0.0", git = "https://github.com/paritytech/substrate.git", branch = "master" , default-features = false}}
sp-keyring = {{ version = "6.0.0", git = "https://github.com/paritytech/substrate.git", branch = "master" }}
sp-runtime = {{ default-features = false, git = "https://github.com/paritytech/substrate", branch = "master" }}
node-template-runtime = {{ git = "https://github.com/paritytech/substrate.git", branch = "master" }}
serde_json = {{ version = "1.0", features = ["raw_value"] }}
serde = {{ version = "1.0", features = ["derive"] }}
env_logger = "0.9.0"
openwhisk-rust = {{ git = "https://github.com/shanithkk/openwhisk-client-rust.git", branch = "master" }}
http = "0.2.8"
bytes = "1"
pallet-staking = {{ git = "https://github.com/paritytech/substrate.git", package = "pallet-staking" ,branch = "master" }}
substrate_macro = {{git = "https://github.com/HugoByte/aurras.git", package = "substrate_macro", branch = "polkadot_macro" }}

"""
    return cargo_dependencies

def global_import_generator(task_list):
    global_imports = ""
    for task in task_list:
        kind = task['kind']
    if kind == "OpenWhisk":
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
    else:
        global_imports = f"""
mod common;
mod traits;
mod types;
use dyn_clone::{{clone_trait_object, DynClone}};
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
use sp_core::H256;
use codec::{{Encode, Decode}};
use sp_runtime::AccountId32;
use substrate_macro::Polkadot;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
pub struct StakingLedger {{
    pub stash: AccountId32,
    #[codec(compact)]
    pub total: u128,
    #[codec(compact)]
    pub active: u128,
    pub unlocking: Vec<u32>,
    pub claimed_rewards: Vec<u32>,
}}
"""
    return global_imports
