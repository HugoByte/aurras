
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
    pub fn init(&mut self, flow: Option<Box<dyn FlowExecutor>>) -> Self {{
        self.edge = flow.to_owned();
        match flow {{
            Some(mut edge) => {{
                if edge.get_input() == None {{
                    match self.vertex.execute(){{
                Ok(()) => (),
                Err(e) => panic!("{{:?}}", e),
            }}
                    edge.set_input_to_the_flow(self.vertex.get_output());
                    edge.set_input_to_task();
                    self.edge = Some(edge);
                }} else {{
                    edge.set_input_to_task();
                    self.edge = Some(edge);
                }}
            }}
            None => match self.vertex.execute(){{
                Ok(()) => todo!(),
                Err(e) => panic!("{{:?}}", e),
            }},
        }}
        self.to_owned()
    }}

    pub fn pipe<'a, T: 'static + FlowExecutor>(&mut self, task: T) -> Workflow {{
        if let Some(edge) = self.edge.to_owned() {{
            let mut workflow = Workflow {{
                vertex: edge.get_flow_task(),
                edge: None,
            }};

            let workflow = workflow.init(Some(Box::new(task))).to_owned();
            return workflow;
        }}
        self.to_owned()
    }}
    pub fn term(&mut self) -> Box<dyn Execute> {{
        if let Some(edge) = self.edge.to_owned() {{
            let mut workflow = Workflow {{
                vertex: edge.get_flow_task(),
                edge: None,
            }};
            
            match workflow.vertex.execute(){{
                Ok(()) => return workflow.vertex,
                Err(e) => panic!("{{:?}}", e),
            }}
            
        }}
        self.vertex.to_owned()
    }}
}}

#[allow(dead_code)]
pub fn concat_function<T: Execute + Clone, U: Execute + Clone, V: FlowExecutor + Clone>(
    work: &Workflow,
    mut task_one: Box<T>,
    mut task_two: Box<U>,
    mut task: V,
) -> impl FlowExecutor {{
    let result: Types;

    match &work.edge{{
        Some(res) => {{
            let mut c_task = res.get_flow_task();
            match c_task.execute(){{
                Ok(()) => result = c_task.get_output(),
                Err(e) => panic!("{{:?}}", e),
            }}
            task_one.set_input(result.to_owned());
            task_two.set_input(result.to_owned());
        }},
        None => {{
            let mut a = work.to_owned();
            match a.vertex.execute(){{
                Ok(()) => result = a.vertex.get_output(),
                Err(e) => panic!("{{:?}}", e),
            }}
            
            task_one.set_input(result.to_owned());
            task_two.set_input(result.to_owned());
        }},
    }}
    let task_one_out :Types;
    let task_two_out :Types;
    match task_one.execute(){{
        Ok(()) => task_one_out = task_one.get_output(),
        Err(e) => panic!("{{:?}}", e),
    }};
    match task_two.execute(){{
        Ok(()) => task_two_out = task_two.get_output(),
        Err(e) => panic!("{{:?}}", e),
    }}

    let concat = ConcatStruct::new(
        task_one_out.try_into().unwrap(),
        task_two_out.try_into().unwrap(),
    );

    task.set_input_to_the_flow(Types::ConCat(concat));
    task
}}

#[allow(dead_code)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone,U: Clone ,V :Clone >(first: HashMap<T,U>, second: HashMap<T,V>)-> HashMap<T,(U,V)>{{

    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first{{
        for (s_key, s_value) in &second{{
            if key.clone() == s_key.to_owned(){{
                data.insert(key.clone(), (value.clone(),s_value.clone()));
            }}
        }}
    }}
    data
}}

#[derive(Clone, Default, Debug)]
pub struct Flow<T: Execute + std::fmt::Debug> {{
    pub input: Option<Types>,
    pub task: T,
}}

impl<T: Execute + Default + Debug + Clone> Flow<T> {{
    pub fn new(task: T) -> Self {{
        Self {{
            task,
            input: Default::default(),
        }}
    }}

    fn output(&mut self) {{
        let result: Types;
        match self.input.clone() {{
            Some(task) => {{
                result = task;
            }}
            None => todo!(),
        }}
        self.task.set_input(result);
    }}
}}
impl<T: 'static + Execute + Debug + Default + Clone> FlowExecutor for Flow<T> {{
    fn set_input_to_task(&mut self) {{
        self.output();
    }}
    fn set_input_to_the_flow(&mut self, task: Types) {{
        self.input = Some(task);
    }}
    fn get_flow_task(&self) -> Box<dyn Execute> {{
        Box::new(self.task.to_owned())
    }}

    fn get_input(&self) -> Option<Types> {{
        self.input.to_owned()
    }}
}}


#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConcatStruct<T, U> {{
    pub first: T,
    pub second: U,
}}

impl<T, U> ConcatStruct<T, U> {{
    pub fn new(first: T, second: U) -> Self {{
        Self {{ first, second }}
    }}
    pub fn get_first(self) -> T {{
        self.first
    }}

    pub fn get_second(self) -> U {{
        self.second
    }}
}}

#[macro_export]
macro_rules! concat {{
    ($ ($struc: expr , $struct2 : expr, $name : ident), *) => {{
        $(
            let $name = ConcatStruct::new($struc, $struct2)
        )*
    }};
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
    fn set_input_to_the_flow(&mut self, task: Types);
    fn get_flow_task(&self) -> Box<dyn Execute>;
    fn set_input_to_task(&mut self);
    fn get_input(&self) -> Option<Types>;
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
