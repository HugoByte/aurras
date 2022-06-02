from ntpath import join
import os
from pydantic import Field
from tackle import BaseHook
from typing import Any, List
import copy

# List to store flows from parsed config
flow_list = []

# List to store tasks from parsed config
task_list = []

task_store = dict()

# Task is a hook to generate rust code from config file


class Task(BaseHook):
    hook_type: str = 'task'
    kind: str = Field(..., title='kind', description="To Specify kind of task")
    name: str = Field(..., title='name',
                      description="Name of the action deployed to openwhisk")
    input_args: List = Field(..., title='input_args',
                             description="List containing dictonary of parameter and type and input")
    output_args: List = Field(..., title='output_args',
                              description="List contating dictonary of paramter and type for output")
    endpoint: str = Field(..., title='endpoint', description="Task Endpoint")

    def exec(self):
        global task_list
        task = {
            "task_name": self.name,
            "kind": self.kind,
            "endpoint_url": self.endpoint,
            "input_args": self.input_args,
            "output_args": self.output_args
        }

        task_list.append(task)

        return


# functions related to task hook for creating rust code

task_struct_impl = f"""use super::*;

"""
impl_task_trait = "trait_impl_task!("
impl_get_task_trait = "trait_impl_getting_task_name!("

implement_run_method = f"""

    fn run(&self) -> Value { 
        
        {

            "self.openwhisk_client().actions().invoke(&self.action_name, serde_json::to_value(self.input.clone()).unwrap(), true, true).unwrap()"
        
        }

    }
"""
task_store_copy = dict()
impl_stucture = ""
setter = ""
new_method = ""
action_properties = dict()

implement_get_action_name_method = f"""
fn get_action_name(&self) -> String {
    {
        "self.action_name.clone()"
    }}
"""


def convert_to_pascalcase(string: str) -> str:

    return string.replace("_", " ").title().replace(" ", "")


def struct_generator():
    global task_list,impl_task_trait,impl_get_task_trait,task_struct_impl,impl_stucture,run,new_method,setter,task_store

    for task in task_list:
        task_dic = dict()
        name = task['task_name']
        kind = task['kind']
        input_args = task['input_args']
        output_args = task['output_args']
        task_name = convert_to_pascalcase(name)
        impl_task_trait += f"{task_name},"
        impl_get_task_trait += f"{task_name},"

        impl_stucture += f"""

impl {task_name} {{
        
        {implement_run_method}
        {implement_get_action_name_method}
}}


"""
        if None not in input_args:
            create_sturct(task_name, input_args, "input", kind)
            for item in input_args:
                field_name = item['name']
                field_type = item['type']

                if field_name != "" and field_type != "":
                    task_dic[field_name] = field_type

        task_store[task_name] = task_dic
        if None not in output_args:
            create_sturct(task_name, output_args, "output", None)

    impl_task_trait = impl_task_trait[:-1]+");"
    impl_get_task_trait = impl_get_task_trait[:-1]+");"

    task_struct_impl += impl_get_task_trait
    task_struct_impl += impl_task_trait

    task_struct_impl += impl_stucture.replace("'", "")
    return


def create_sturct(task_name: str, args: list, type: str, kind):
    global task_struct_impl
    global impl_task_trait
    global action_properties
    action_prop = ""

    for property in action_properties['action']:
        if convert_to_pascalcase(property['name']) == task_name:
            action_prop += f"""
#[AuthKey="{property['auth_token']}"]
#[ApiHost="{property['api_host']}"]
#[Insecure="{property['insecure'].lower()}"]
#[Namespace="{property['namespace']}"]
            """

    argument = ""
    for item in args:

        field_name = item['name']
        field_type = item['type']

        if field_name != "" and field_type != "":
            argument += f"""
        {field_name}:{field_type},
            """

    if type == "input":
        task_struct_impl += f"""

#[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]
{action_prop}
pub struct {task_name}{{
    action_name: String,
    pub input:{task_name}{type.title()},
}}
    """

    if argument != "":
        task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct {task_name}{type.title()} {{
{argument} 
}}
"""

    task_struct_impl = task_struct_impl.replace("\\", "")
    task_struct_impl = task_struct_impl.rjust(len(task_struct_impl))

    return


# FlowHook for parsing the config and genrating required rust code from it
generic_input_sturct_filed = ""


class Flow(BaseHook):
    hook_type: str = 'flow'
    type: str = Field(..., description="Type of the flow")
    task_name: str = Field(..., description="Task Name")
    depends_on: Any = Field(..., description="Dependent Task")

    def exec(self):
        flow = {
            "type": self.type,
            "task_name": convert_to_pascalcase(self.task_name),
            "depends_on": self.depends_on,
        }

        flow_list.append(flow)

        return


dependency_matrix = dict()


def create_main_input_struct():
    global flow_list, task_list, task_store, task_struct_impl, new_method, generic_input_sturct_filed, task_store_copy, dependency_matrix
    task_store_copy = copy.deepcopy(task_store)
    setter = dict()
    dependency = dict()

    dep_field = dict()
    dep_task = []
    depend_task_type_map = ""
    current_task_type_map = ""
    map_task_name = ""

    local_pipe_dic = dict()
    local_pipe_list = []

    for flow in flow_list:

        if flow['type'] == "Init":

            # revisit
            for key, values in task_store_copy[flow['task_name']].items():
                generic_input_sturct_filed += f"pub {key}:{values},"
            if flow['depends_on'] == None:
                # dependency_matrix[flow['type']] = [flow['task_name']]
                local_pipe_list.append(flow['task_name'])
                dependency_matrix[flow['type']] = local_pipe_list

        elif flow['type'] == "Pipe" or flow['type'] == "Term":

            for item in flow['depends_on']['task']:
                local_pipe_list_pipe = []
                local_pipe_list_term = []
                dep_field[item['name']] = item['fields']

                poped = dict()
                for filed in item['fields']:
                    poped[filed] = task_store_copy[flow['task_name']].pop(
                        filed)
                setter[flow['task_name']] = poped
                poped = dict()
                dependency[flow['task_name']] = item['name'].replace(
                    "_", " ").title().replace(" ", "")

                # dependency_matrix[flow['Term']] =[]
                if flow['type'] == "Pipe":
                    dep_task.append(item['name'])
                    local_pipe_dic[flow['task_name']] = dep_task
                    local_pipe_list_pipe.append(local_pipe_dic)
                    dependency_matrix['Pipe'] = local_pipe_list_pipe
                    local_pipe_list = []
                    # local_pipe_dic = dict()
                else:
                    local_pipe_dic = dict()
                    dep_task.append(item['name'])
                    local_pipe_dic[flow['task_name']] = dep_task

                    local_pipe_list_term.append(local_pipe_dic)
                    dependency_matrix['Term'] = local_pipe_list_term
                    local_pipe_list = []

            dependency[flow['task_name']] = dep_field
            dep_field = dict()

            for key, values in task_store_copy[flow['task_name']].items():
                generic_input_sturct_filed += f"pub {key}:{values},"
            # dependency_matrix[flow['type']] = local_pipe_dic
            # dependency_matrix[flow['type']] =[]
            # dependency_matrix[flow['type']].append(local_pipe_list)
            # local_pipe_list = []
            dep_task = []

        else:

            for item in flow['depends_on']['task']:
                map_task_name = flow['task_name']
                depends_on_task = item['name'].replace(
                    "_", " ").title().replace(" ", "")

                for task in task_list:
                    if map_task_name == convert_to_pascalcase(task['task_name']):
                        for i in task['output_args']:
                            current_task_type_map = i['type']
                    elif depends_on_task == convert_to_pascalcase(task['task_name']):
                        for i in task['output_args']:
                            depend_task_type_map = i['type']

                if depend_task_type_map == "Vec<String>":
                    depend_task_type_map = "String"

            task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct {map_task_name }MapOutput{{
    model_prize_list : HashMap<{depend_task_type_map},{current_task_type_map}>
}}
"""

        # local_pipe_dic = dict()
        # dependency_matrix[flow['type']] = local_pipe_list
        local_pipe_list = []

    task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Input{{
    {generic_input_sturct_filed.replace("'", "").replace("{", "").replace("}", "")}
}}
"""

    implement_new_and_setter(task_list, task_store_copy, setter, dependency)
    return


def implement_new_and_setter(task_list, task_store_copy, setter, dependency):
    global task_struct_impl

    for task in task_list:
        task_name = convert_to_pascalcase(task['task_name'])
        method_param = f"{task_store_copy[task_name]}".replace(
            "'", "").replace("{", "").replace("}", "")
        field_assign = ",".join(list(task_store_copy[task_name].keys()))
        if field_assign != "" and method_param != "":
            field_assign += ","
            method_param += ","
        new_impl_methods = f"""
 pub fn new({method_param}action_name:String) -> Self {{ Self{{  input:{task_name}Input{{{field_assign} ..Default::default()}},action_name: action_name, ..Default::default()}}}}
"""
        new_impl_methods = new_impl_methods.strip().replace("'", "")
        new_impl = ""
        setter_method = ""
        if task_name in setter.keys():
            if task_name in setter.keys():
                fields = setter[task_name].keys()
                set_fileds = ""
                for x in fields:
                    set_fileds += f"""self.input.{x} = value.{x};"""
            dep_task_out = convert_to_pascalcase(
                " ".join(dependency[task_name].keys()))
            setter_method += f"""fn setter(&mut self,value:{dep_task_out}Output){{{set_fileds}}}""".replace(
                "\\n", "").replace("'", "")
            new_impl += f"""
impl {task_name} {{

{new_impl_methods}
{setter_method}

}}
"""
            task_struct_impl += new_impl.replace("'", "")
        else:
            setter_method += "fn setter(&mut self){}"
            new_impl += f"""
impl {task_name} {{

{new_impl_methods}
{setter_method}

}}
"""
            task_struct_impl += new_impl.replace("'", "")

    return

main_file = ""

def create_main_function(task):
    global task_store, task_store_copy, dependency_matrix, task_struct_impl, global_imports,main_file
    main = ""
    flow = ""
    dep_matrix = ""
    initilization = ""
    workflow = ""
    setter_trait = ""

    for key, values in task_store_copy.items():
        new_fileds = ""
        for value in values:
            new_fileds += f"input.{value},"

        initilization += f"""
        let mut {key.lower()} = {key}::new({new_fileds}String::from("{key.lower()}"));
            """
    for key, values in dependency_matrix.items():

        if "Init" in key:
            workflow += f"""workflow.init(&mut {convert_to_pascalcase("".join(values)).lower()})"""
        if "Pipe" in key or "Term" in key:
            for value in values:
                for k, v in value.items():
                    if key == "Pipe":
                        workflow += f""".pipe(&mut {k.lower()}_list)"""
                    else:
                        workflow += f""".term(&mut {k.lower()}_list)"""
                    dep_matrix += f"""
                  dependncy_matrix.insert("{convert_to_pascalcase(k).lower()}".to_string(), "{convert_to_pascalcase("".join(v)).lower()}".to_string());
                 """
                    setter_trait += f"""
                    impl Setting for {k}{{
                        fn setting(&mut self,value: Self::Input) {{
                        self.setter(value)
                    }}
                    type Input = {convert_to_pascalcase("".join(v))}Output;
                    }}
                    
                    """
                    flow += f"""
let mut {k.lower()}_list : Flow<{k},{convert_to_pascalcase("".join(v))}Output> = Flow::new({k.lower()});
"""

    main += f"""
    {global_imports}    

    pub fn main(args:Value) -> Result<Value,Error>{{
    let input: Input = serde_json::from_value(args)?;
    {initilization}
    {flow}
    let mut dependncy_matrix: HashMap<String, String> = HashMap::new();
    {dep_matrix}
    let mut workflow = WorkFlows::new(dependncy_matrix);
    let result = {workflow};
    Ok(result.unwrap())
}}
    """
    task_struct_impl += setter_trait
    main_file += main
    
class WorkFlow(BaseHook):
    hook_type: str = 'workflow'
    name: str = Field(..., title='name', description='Workflow name')
    version: str = Field(..., title='version', description='Wrokflow Version')
    action_properties: Any = Field(..., title="action_properties",
                                   description="Porperties for the actions defined")

    def exec(self):
        global task_list
        global task_store_copy
        global action_properties

        action_properties = self.action_properties
        struct_generator()
        create_main_input_struct()
        create_main_function(task_list)
        generate_output()

        return


def generate_output():
    global cargo_file, common_rs_file, traits_file, task_struct_impl,main_file
    output_path = "../../"
    path = os.path.join(output_path, "output/src")
    os.makedirs(path, mode=0o777)
    cargo = open(os.path.join(output_path, "output/Cargo.toml"), 'w')
    cargo.writelines(cargo_file)
    cargo.close()

    
    rustfile = open(os.path.join(output_path, "output/src/common.rs"), 'w')
    rustfile.writelines(common_rs_file)
    rustfile.close()
    rustfile = open(os.path.join(output_path, "output/src/traits.rs"), 'w')
    rustfile.writelines(traits_file)
    rustfile.close()
    rustfile = open(os.path.join(output_path, "output/src/types.rs"), 'w')
    rustfile.writelines(task_struct_impl)
    rustfile.close()
    rustfile = open(os.path.join(output_path, "output/src/lib.rs"), 'w')
    rustfile.writelines(main_file)
    rustfile.close()



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

files = [{
    "common.rs" : common_rs_file,
    "trait.rs" : traits_file,
    "lib.rs" : main_file,
    "types.rs" : task_struct_impl

}]