

cargo_file = f"""
[package]
name = "action-workflow"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = "1.0.137"
serde_json = "1.0.81"
serde_derive = "1.0.81"
paste = "1.0.7"
openwhisk-rust = "0.1.0"
openwhisk_macro = "0.1.0"
yaml-rust = "0.4.1"
serde_yaml = "0.8.24"
"""
common_rs_file = f"""
use serde_json::Error;

use super::*;

#[derive(Debug, Default)]
pub struct WorkFlows {{
    output: HashMap<String, Value>,
    dependency_matrix: HashMap<String, String>,
}}

impl WorkFlows {{
    pub fn new(dependency_matrix: HashMap<String, String>) -> Self {{
        Self {{
            dependency_matrix,
            ..Default::default()
        }}
    }}

    pub fn init<T: Execute + Getter>(&mut self, mut task: &mut T) -> &mut Self {{
        let some_key = task.get_action_name();
        let a = task.execute();
        self.output.insert(some_key, a);
        self
    }}

    pub fn pipe<T: Executor + Clone>(&mut self, mut task: &mut T) -> &mut Self {{
       
        let key =  task.get_action_name_task();
        let value = self.dependency_matrix.get(&key).unwrap();
        let value = self.output.get(value).unwrap();
        task.deserialize_output(value);
        let result = task.executor_execute();
        self.output.insert(key.to_string(), result);
        self
    }}

    pub fn term<T: Executor + Clone>(&mut self, mut task: &mut T) -> Result<Value, Error> {{
        let key =  task.get_action_name_task();
        let value = self.dependency_matrix.get(&key).unwrap();
        let value = self.output.get(value).unwrap();
        task.deserialize_output(value);
        let result = task.executor_execute();
        self.output.insert(key.to_string(), result.clone());
        Ok(result)
    }}
}}

#[macro_export]
macro_rules! trait_impl_task{{
    ($ ($call:ty) ,*) => {{
        paste! {{
            $( impl Execute for [<$call>]{{
                type Output = Value;
                fn execute(&self) -> Value {{
                    self.run()
                }}
            }}
            )*
        }}
    }}
}}
#[macro_export]
macro_rules! trait_impl_getting_task_name{{
    ($ ($call:ty) ,*) => {{
        paste! {{
            $( impl Getter for [<$call>]{{
                fn get_action_name(&self) -> String {{
                    self.get_action_name()
                }}
            }}
            )*
        }}
    }}
}}


"""
traits_file = f"""
use super::*;

pub trait Execute {{
    type Output: Clone + Default + Debug;
    fn execute(&self) -> Value;
}}

pub trait Executor {{
    type Output;
    fn executor_execute(&mut self) -> Value;
    fn deserialize_output(&mut self, value: &Value);
    fn get_action_name_task(&self) ->String;
}}

#[derive(Clone, Default, Debug)]
pub struct Flow<T: Execute + std::fmt::Debug + Default + Setting, U: Clone> {{
    input: U,
    output: <T as Execute>::Output,
    task: T,
}}

impl<T: Execute + Default + Debug +Setting, U: Clone + Default + for<'de> Deserialize<'de>> Flow<T, U> {{
    pub fn new(task: T) -> Self
    where
        <T as traits::Execute>::Output: Default,
    {{
        Self {{
            output: Default::default(),
            task,
            input: Default::default(),
        }}
    }}

    pub fn deserialize(&mut self, value: Value) {{
        self.input = serde_json::from_value(value).unwrap();
    }}
}}

impl<
        'a,
        T: traits::Setting + Execute + Debug + Default + Clone+ Getter,
        U: Clone + Default + for<'de> Deserialize<'de>,
    > Executor for Flow<T, U>
where
    for<'de> <T as traits::Execute>::Output: Deserialize<'de>,
    T: traits::Setting<Input = U>,
{{
    type Output = Flow<T, U>;
    fn executor_execute(&mut self) -> Value {{
        self.task.execute()
    }}
    fn deserialize_output(&mut self, value: &Value) {{
        self.deserialize(value.to_owned());
        self.task.setting(self.input.clone())
    }}
    fn get_action_name_task(&self) ->String {{
        self.task.get_action_name()
    }}
}}

pub trait Setting {{
    type Input: Clone + Default + Debug;
    fn setting(&mut self, value: Self::Input);
}}

pub trait Getter {{
    fn get_action_name(&self) -> String;
}}
"""

global_imports = f"""
mod common;
mod traits;
mod types;
use common::*;
use openwhisk_macro::OpenWhisk;
use openwhisk_rust::*;
use paste::paste;
use std::fmt::Debug;
use traits::*;
use types::*;

use serde::{{
    Deserialize, Serialize,
}};

use serde_json::{{Error, Value}};
use std::collections::HashMap;
"""
