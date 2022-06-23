
dependencies = f"""
[dependencies]
serde = "1.0.137"
serde_json = "1.0.81"
serde_derive = "1.0.81"
paste = "1.0.7"
dyn-clone = "1.0"
derive-enum-from-into = "0.1.1"
openwhisk-rust = "0.1.1"
openwhisk_macro = "0.1.4"

"""
common_rs_file = f"""
use super::*;

#[derive(Debug, Clone)]
pub struct Workflow {{
    pub vertex: Box<dyn Execute>,
    pub edge: Option<Box<dyn FlowExecutor>>,
}}

impl Workflow {{
    pub fn new<T: 'static + Execute>(task: T) -> Self {{
        Self {{
            vertex: Box::new(task),
            edge: None,
        }}
    }}
    pub fn init(&mut self, flow: Option<Box<dyn FlowExecutor>>) -> &mut Self {{
        self.vertex.execute();
        self.edge = flow;
        match self.edge.clone() {{
            Some(mut edge) => {{
                edge.set_input_to_the_flow(self.vertex.to_owned());
                edge.set_input_to_task();
                self.edge = Some(edge);
            }}
            _ => (),
        }}
        self
    }}
    pub fn pipe<T: 'static + FlowExecutor + Clone>(&mut self, task: T) -> Workflow {{
        if let Some(edge) = self.edge.clone() {{
            let mut workflow = Workflow {{
                vertex: edge.get_flow_task(),
                edge: None,
            }};

            let workflow = workflow.init(Some(Box::new(task))).to_owned();
            return workflow;
        }}
        self.clone()
    }}
    pub fn term(&mut self) -> Box<dyn Execute> {{
        if let Some(edge) = self.edge.clone() {{
            let mut workflow = Workflow {{
                vertex: edge.get_flow_task(),
                edge: None,
            }};
            workflow.vertex.execute();
            return workflow.vertex;
        }}
        self.vertex.clone()
    }}
}}

#[derive(Debug, Clone, Default)]
pub struct Flow<T: Execute + Debug + Default + Clone> {{
    input: Option<Box<(dyn traits::Execute + 'static)>>,

    task: T,
}}

impl<T: Execute + Default + Debug + Clone> Flow<T> {{
    pub fn new(task: T) -> Self {{
        Self {{
            task,
            input: Default::default(),
        }}
    }}

    fn output(&mut self) {{
        let output: Types;
        match self.input.clone() {{
            Some(task) => {{
                output = task.get_output();
            }}
            None => todo!(),
        }}
        self.task.set_input(output);
    }}
}}

impl<T: 'static + Execute + Debug + Default + Clone> FlowExecutor for Flow<T> {{
    fn set_input_to_the_flow(&mut self, task: Box<dyn Execute>) {{
        self.input = Some(task);
    }}

    fn get_flow_task(&self) -> Box<dyn Execute> {{
        Box::new(self.task.clone())
    }}

    fn set_input_to_task(&mut self) {{
        self.output();
    }}
}}

"""
traits_file = f"""
use std::fmt::Debug;
use super::*;

pub trait Execute: Debug + DynClone {{
    fn execute(&mut self) -> Result<(),String>;
    fn get_output(&self) -> Types;
    fn set_input(&mut self, inp: Types);
}}

clone_trait_object!(Execute);

pub trait FlowExecutor: Debug + DynClone {{
    fn set_input_to_the_flow(&mut self, task: Box<dyn Execute>);
    fn get_flow_task(&self) -> Box<dyn Execute>;
    fn set_input_to_task(&mut self);
}}

clone_trait_object!(FlowExecutor);


#[macro_export]
macro_rules! impl_execute_trait {{
    ($ ($struct : ty), *) => {{
        
            paste!{{
                $( impl Execute for $struct {{
                    fn execute(&mut self) -> Result<(),String> {{
                        self.run()
                    }}
                
                    fn get_output(&self) -> Types {{
                        Types::$struct(self.get_task_output())
                    }}
                
                    fn set_input(&mut self, input: Types) {{
                        self.setter(input.try_into().unwrap())
                    }}
                }}
            )*
        }}
    }};
}}

"""

global_imports = f"""
mod traits;
mod types;
mod common;
use openwhisk_macro::OpenWhisk;
use openwhisk_rust::*;
use paste::paste;
use std::fmt::Debug;
use traits::*;
use types::*;
use common::*;
use dyn_clone::{{clone_trait_object,DynClone}};
use serde::{{
    Deserialize, Serialize,
}};
use serde_json::{{Error, Value}};
use std::collections::HashMap;
use std::convert::TryInto;
use derive_enum_from_into::{{EnumFrom, EnumTryInto}};
"""
