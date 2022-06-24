import copy
import os
import re
from .constants import dependencies,common_rs_file,traits_file,global_imports


#global variables
create_enum = f"""
"""
task_struct_impl = f"""use super::*;
"""
impl_task_trait = "impl_execute_trait!("
task_store = dict()
task_store_copy = dict()
impl_stucture = ""
setter = ""
new_method = ""
action_properties = dict()
generic_input_sturct_filed = ""
dependency_matrix = dict()
main_file = ""

#to convert camel_case to PascalCase
def convert_to_pascalcase(string: str) -> str:

    return string.replace("_", " ").title().replace(" ", "")

"""
    Creates Cargo.toml file using workflow config
        # Arguments
            `name`    - worklfow package name
            `version` - workflow version
"""

def create_workflow_config(name,version) -> str:
    workflow_config = f"""
[package]
name = "{name}"
version = "{version}"
edition = "2018"

"""
    return workflow_config

"""
    Creates rust struct type based on the YAML config provided
        # Arguments
            `task_list`    - A list of dictonary containing parsed yaml conifg from Task Hook
            `action_props` - A list of dictonary containing parsed action properties from Workflow Hook

"""
def struct_generator(task_list,action_props):
    global impl_task_trait,impl_get_task_trait,task_struct_impl,impl_stucture,run,new_method,setter,task_store,create_enum
    enum =""
    
    for task in task_list:
        task_dictionary = dict()
        name = task['task_name']
        kind = task['kind']
        input_args = task['input_args']
        output_args = task['output_args']
        task_name = name
        impl_task_trait += f"{task_name},"
        
        if None not in input_args:
            create_sturct(task_name, input_args, "input", kind,action_props)
            for item in input_args:
                field_name = item['name']
                field_type = item['type']

                if field_name != "" and field_type != "":
                    task_dictionary[field_name] = field_type
        
        task_store[task_name] = task_dictionary
        if None not in output_args:
            create_sturct(task_name, output_args, "output", None,action_props)

    enum += f"""
       #[derive(EnumFrom, EnumTryInto, PartialEq, Debug, Clone,Serialize,Deserialize)]
pub enum Types{"{"}
    Empty(String),
    ConCat(ConcatStruct<String,String>),
    {create_enum}
    """
    impl_task_trait = impl_task_trait[:-1]+");"
    task_struct_impl += impl_task_trait
    enum = enum[:-1]+"}"
    task_struct_impl += enum
    return

"""
    Creates input and output struct
        # Arguments
            `task_name`    - name of the task
            `args` - List of arguments
            `type` - Spcifies Input or Ouput type
            `kind` - Represents type of actions
            `action_properties - Properties of the action from yaml config

"""
def create_sturct(task_name: str, args: list, type: str, kind,action_properties):
    global task_struct_impl
    global impl_task_trait
    global create_enum
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
        create_enum += f"""
        {task_name}({task_name}Output),
    """
        task_struct_impl += f"""

#[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]
{action_prop}
pub struct {task_name}{{
    action_name: String,
    pub input:{task_name}{type.title()},
    pub output:{task_name}Output,
}}
    """
    
    if argument != "":
        task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct {task_name}{type.title()} {{
{argument} 
}}
"""
    task_struct_impl = task_struct_impl.replace("\\", "")
    task_struct_impl = task_struct_impl.rjust(len(task_struct_impl))

    return

"""
    Creates Main Input struct
        # Arguments
            `task_list` - A list of dictonary containing parsed yaml conifg from Task Hook
            `flow_list` - A list of dictonary containing parsed yaml conifg from Flow Hook

"""

def create_main_input_struct(task_list,flow_list):
    global  task_store, task_struct_impl, new_method, generic_input_sturct_filed, task_store_copy, dependency_matrix
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
            for field, type in task_store_copy[flow['task_name']].items():
                generic_input_sturct_filed += f"pub {field}:{type},"
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
            dep_task = []

        else:

            for item in flow['depends_on']['task']:
                map_task_name = flow['task_name']
                depends_on_task = item['name'].replace(
                    "_", " ").title().replace(" ", "")

                for task in task_list:
                    if map_task_name == task['task_name']:
                        for i in task['output_args']:
                            current_task_type_map = i['type']
                    elif depends_on_task == task['task_name']:
                        for i in task['output_args']:
                            depend_task_type_map = i['type']

                if depend_task_type_map != None:
                    dep_type = re.findall(r'\<.*?\>', depend_task_type_map)
                    depend_task_type_map = "".join(dep_type)

            task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct {map_task_name }MapOutput{{
    field_name : HashMap<{depend_task_type_map},{current_task_type_map}>
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

"""
    Generates new and Setter method for respective structure
"""
def implement_new_and_setter(task_list, task_store_copy, setter, dependency):
    global task_struct_impl

    for task in task_list:
        task_name = task['task_name']
        method_param = f"{task_store_copy[task_name]}".replace(
            "'", "").replace("{", "").replace("}", "")
        field_assign = ",".join(list(task_store_copy[task_name].keys()))
        if field_assign != "" and method_param != "":
            field_assign += ","
            method_param += ","
        new_impl_methods = f"""
 pub fn new({method_param}action_name:String) -> Self {{ Self{{  input:{task_name}Input{{{field_assign} ..Default::default()}},action_name: action_name, ..Default::default()}}}}
"""
        new_get_task_output = f"""
fn get_task_output(&self) ->{task_name}Output{{
    self.output.clone()
}}
"""     
        new_get_task_output = new_get_task_output.strip().replace("'","")
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
{new_get_task_output}

}}
"""
            task_struct_impl += new_impl.replace("'", "")
        else:
            setter_method += "fn setter(&mut self,_value:String){}"
            new_impl += f"""
impl {task_name} {{

{new_impl_methods}
{setter_method}
{new_get_task_output}
}}
"""
            task_struct_impl += new_impl.replace("'", "")

    return


"""
    Creates main function to use and run workflow generated from yaml config
"""
def create_main_function(task):
    global task_store, task_store_copy, dependency_matrix, task_struct_impl, global_imports,main_file
    main = ""
    flow = ""
    dependency_matrix_map = ""
    initilization = ""
    workflow = ""
    workflow_dag = ""
    result = ""
    # setter_trait = ""

    for key, values in task_store_copy.items():
        new_fileds = ""
        for value in values:
            new_fileds += f"input.{value},"

        initilization += f"""
        let {key.lower()} = {key}::new({new_fileds}String::from("{key.lower()}"));
            """
    for key, values in dependency_matrix.items():

        if "Init" in key:
            workflow_dag += f""" Workflow::new({convert_to_pascalcase("".join(values)).lower()});"""
            
        if "Pipe" in key:
            pipe_task = list(values[0].keys())
            workflow += f"""workflow.init(Some(Box::new({convert_to_pascalcase("".join(pipe_task[0])).lower()}_flow)))"""
            for key in pipe_task[1:]:
                workflow  += f""".pipe({key.lower()}_flow)"""
            for key in pipe_task:
                flow += f"""
let {key.lower()}_flow = Flow::new({key.lower()});
"""

        if "Term" in key:
            keys = list(values[0].keys())
            for key in keys:
                workflow += f""".pipe({key.lower()}_flow).term()"""
                flow += f"""
let {key.lower()}_flow = Flow::new({key.lower()});
"""
                result += f"""
let result: {key}Output = result.get_output().try_into().unwrap();
let result = serde_json::to_value(result).unwrap();
Ok(result)
"""
    main += f"""
    {global_imports}    

    pub fn main(args:Value) -> Result<Value,Error>{{
    let input: Input = serde_json::from_value(args)?;
    {initilization}
    {flow}
    let mut workflow = {workflow_dag}
    let result = {workflow};
    {result}
    
}}
    """
    # task_struct_impl += setter_trait
    main_file += main

"""
    To Write generated code to Rust package 
"""
def generate_output(workflow_config: str):
    global dependencies, common_rs_file, traits_file, task_struct_impl,main_file
    workflow_config += dependencies

    output_path = "../../"
    path = os.path.join(output_path, "output/src")
    os.makedirs(path, mode=0o777)
    cargo = open(os.path.join(output_path, "output/Cargo.toml"), 'w')
    cargo.writelines(workflow_config)
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
