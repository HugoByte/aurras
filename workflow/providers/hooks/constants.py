
cargo_dependencies = f"""
[lib]
crate-type = ["cdylib"]

[dependencies]
serde = "1.0.137"
serde_json = "1.0.81"
serde_derive = "1.0.81"
derive-enum-from-into = "0.1.1"
openwhisk-rust = "0.1.2"
openwhisk_macro = "0.1.5"
paste = "1.0.7"
dyn-clone = "1.0.7"

"""
common_rs_file = f"""
use super::*;
#[derive(Debug)]
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

    pub fn node_count(&self) -> usize {{
        self.nodes.len()
    }}

    pub fn add_node(&mut self, task: Box<dyn Execute>) -> usize {{
        let len = self.nodes.len();
        self.nodes.push(task);
        len
    }}

    pub fn add_edge(&mut self, parent: usize, child: usize) {{
        self.edges.push((parent, child));
    }}

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {{
        edges
            .iter()
            .for_each(|(source, destination)| self.add_edge(*source, *destination));
    }}

    pub fn get_task(&self, index: usize) -> &Box<dyn Execute> {{
        self.nodes.get(index).unwrap()
    }}

    pub fn get_task_as_mut(&mut self, index: usize) -> &mut Box<dyn Execute> {{
        self.nodes.get_mut(index).unwrap()
    }}

    pub fn node_indices(&self) -> Vec<usize> {{
        (0..self.node_count()).collect::<Vec<_>>()
    }}

    pub fn init(&mut self) -> Result<&mut Self,String> {{
        match self.get_task_as_mut(0).execute(){{
            Ok(()) => Ok(self),
            Err(err) => Err(err)
        }}
       
    }}
    pub fn term(&mut self, task_index: Option<usize>) -> Result<Types,String> {{

        match task_index {{
            Some(index) => {{
                let previous_index = (index - 1).try_into().unwrap();
                let previous_task = self.get_task(previous_index);
                let previous_task_output = previous_task.get_task_output();
                let current_task = self.get_task_as_mut(index);
                current_task.set_output_to_task(previous_task_output);
                match current_task.execute(){{
                    Ok(()) => Ok(current_task.get_task_output()),
                    Err(err) => Err(err),
                }}
                
            }},
            None => {{
                let len = self.node_count();
                Ok(self.get_task(len-1).get_task_output())
            }},
        }}
        
    }}

    pub fn pipe(&mut self, task_index: usize) -> Result<&mut Self,String> {{
        let mut list = Vec::new();
        let edges_list = self.edges.clone();
        edges_list.iter().for_each(|(source, destination)| {{
            if destination == &task_index {{
                list.push(source)
            }}
        }});
        let mut res: Vec<Types> = Vec::new();
        match list.len() {{
            0 => {{
                match self.get_task_as_mut(task_index).execute(){{
                    
                    Ok(()) => Ok(self),
                    Err(err) => Err(err),
        
                }}
            }},
            1 => {{
                let previous_task_output = self.get_task(*list[0]).get_task_output();
                let current_task = self.get_task_as_mut(task_index);
                current_task.set_output_to_task(previous_task_output);
                match current_task.execute(){{
                Ok(()) => Ok(self),
                Err(err) => Err(err),
                }}
            }}
            _ => {{
                res = list
                    .iter()
                    .map(|index| {{
                        let previous_task = self.get_task(**index);
                        let previous_task_output = previous_task.get_task_output();
                        previous_task_output
                    }})
                    .collect();

                let s: Types = res.into();
                let current_task = self.get_task_as_mut(task_index);
                current_task.set_output_to_task(s);
                
                match current_task.execute(){{
                Ok(()) => Ok(self),
                Err(err) => Err(err),
        }}
            }}
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

use std::convert::TryInto;
use paste::*;
use common::*;
use traits::*;
use types::*;
"""
