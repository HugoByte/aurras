import copy
from dataclasses import field, fields
from mimetypes import init
import os
from .constants import common_rs_file, traits_file,run_function
from .common import *
from .constants import cargo_generator, global_import_generator

#global variables
create_enum = f"""
"""
task_struct_impl = f"""use super::*;
"""
impl_task_trait = "impl_execute_trait!("

impl_stucture = ""
setter = ""
new_method = ""
action_properties = dict()
generic_input_sturct_filed = ""
dependency_matrix = dict()
main_file = ""
map_task_name = []
dependencies = dict()
main_input_dict = dict()


"""
    Creates Cargo.toml file using workflow config
        # Arguments
            `name`    - worklfow package name
            `version` - workflow version
"""


def create_workflow_config(name, version) -> str:
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


def struct_generator(task_list):
    global map_task_name, impl_task_trait, impl_get_task_trait, task_struct_impl, impl_stucture, new_method, create_enum, dependencies
    enum = ""
    for task in task_list:
        task_dictionary = dict()
        name = task['task_name']
        kind = task['kind']
        input_args = task['input_args']
        output_args = task['output_args']
        task_name = convert_to_pascalcase(name)
        props = task['properties']
        impl_task_trait += f"{task_name},"

        if None not in input_args:
            create_sturct(task_name, input_args, "input")
        if None not in output_args:
            create_sturct(task_name, output_args, "output")
        
        if map_task_name == []:
                task_struct_impl += create_main_struct(
                    task_name, props, "", kind)

        else:
            if task_name in map_task_name :
                    task_struct_impl += create_main_struct(
                        task_name, props, "map", kind)
            else:
                    task_struct_impl += create_main_struct(
                        task_name, props, "", kind)
        if task_name in map_task_name:
            create_enum += f"""
{task_name}(Mapout{task_name}),
"""
            type_in = ""
            type_out = ""
            for item in input_args:
                type_in += item['type']
            for item in output_args:
                type_out += item['type']
            task_struct_impl += f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct Mapout{task_name}{{
    result : HashMap<{type_in},{type_out}>
}}
"""
            type_in = ""
            type_out = ""
        else:
            create_enum += f"""
{task_name}({task_name}Output),
"""

    enum += f"""
#[derive(EnumFrom, EnumTryInto, PartialEq, Debug, Clone,Serialize,Deserialize)]
pub enum Types{"{"}
    Empty(String),
    Concat(Vec<Types>),
    {create_enum}
    """
    impl_task_trait = impl_task_trait[:-1]+");"
    task_struct_impl += impl_task_trait
    enum = enum[:-1]+"}"
    task_struct_impl += enum
    implement_new_and_setter(task_list, dependencies)
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


def create_sturct(task_name: str, args: list, type: str):
    global task_struct_impl
    global impl_task_trait
    global create_enum
    global map_task_name

    argument = ""
    for item in args:

        field_name = item['name']
        field_type = item['type']

        if field_name != "" and field_type != "":
            argument += f"""
        {field_name}:{field_type},
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

def create_main_struct(task_name, properties, type, kind) -> str:
    task_struct_impl = ""
    action_prop = ""
    if kind == "OpenWhisk":
        action_prop += f"""
        #[AuthKey="{properties['auth_token']}"]
        #[ApiHost="{properties['api_host']}"]
        #[Insecure="{properties['insecure'].lower()}"]
        #[Namespace="{properties['namespace']}"]
        """
        if type == "map":
            task_struct_impl += f"""
            #[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]
            {action_prop}
            pub struct {convert_to_pascalcase(task_name)}{{
                action_name: String,
                pub input:{task_name}Input,
                pub output:{task_name}Output,
                pub mapout: Mapout{task_name},
            }}
            """
        else:
            task_struct_impl += f"""
            #[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]
            {action_prop}
            pub struct {task_name}{{
                action_name: String,
                pub input:{task_name}Input,
                pub output:{task_name}Output,
            }}
            """
    else:
        action_prop += f"""
        #[Chain="{properties['Chain']}"]
        #[Operation="{properties['Operation']}"]
        """
        if type == "map":
            task_struct_impl += f"""
            #[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]
            {action_prop}
            pub struct {convert_to_pascalcase(task_name)}{{
                action_name: String,
                pub input:{task_name}Input,
                pub output:{task_name}Output,
                pub mapout: Mapout{task_name},
            }}
            """
        else:
            task_struct_impl += f"""
            #[derive(Default, Debug, Clone, Serialize, Deserialize,{kind})]{action_prop}pub struct {task_name}{{
                action_name: String,
                pub input:{task_name}Input,
                pub output:{task_name}Output,
            }}
            """
    return task_struct_impl




"""

    Generates Dependency Matrix based on the flow
        # Arguments 
        `flow_lis` - A list of dictonary containing parsed yaml conifg from Task Hook
"""


def generate_dependency_matrix(flow_list):
    global task_store, map_task_name, dependencies, task_struct_impl, new_method, generic_input_sturct_filed, dependency_matrix

    map_tasks = []
    task_pipe = []
    task_concat = []
    task_term = []
    task_init = []

    for flow in flow_list:
        if flow['type'] == "Init":
            if flow['depends_on'] == None:
                dependency = {
                    "task_name": flow['task_name'],
                    "dependent_task": "",
                }
            task_init.append(dependency)
            dependencies['init'] = {"no_op": task_init}

        elif flow['type'] == "Pipe" or flow['type'] == "Term":
            if flow['type'] == "Pipe":

                if flow['depends_on']['operation'] == "map":
                    dependency = {
                        "task_name": flow['task_name'],
                        "dependent_task": flow['depends_on']['task'],
                    }
                    map_task_name.append(flow['task_name'])

                    map_tasks.append(dependency)
                elif flow['depends_on']['operation'] == "concat":
                    dependency = {
                        "task_name": flow['task_name'],
                        "dependent_task": flow['depends_on']['task'],
                    }
                    task_concat.append(dependency)
                else:
                    dependency = {
                        "task_name": flow['task_name'],
                        "dependent_task": flow['depends_on']['task'],
                    }
                    task_pipe.append(dependency)
            else:
                dependency = {
                    "task_name": flow['task_name'],
                    "dependent_task": flow['depends_on']['task'],
                }
                task_term.append(dependency)

            dependencies['pipe'] = {"map": map_tasks,
                                    "concat": task_concat, "no_op": task_pipe}
            dependencies['term'] = {"no_op": task_term}
    return


"""
    Generates new and Setter method for respective structure
"""


def implement_new_and_setter(task_list, dependency):
    global task_struct_impl
    global map_task_name
    global main_input_dict
    generic_input_struct = ""
    dep_task = []
    for task in task_list:
        task_name = convert_to_pascalcase(task['task_name'])
        for key, values in dependency.items():

            if key == "init":
                input_fields = []
                if values['no_op'][0]['task_name'] == task_name:
                    setter_method = "fn setter(&mut self,_value:Types){}"
                    output_method = out_put_method("", task_name)
                    method_param = ""
                    field_assign = ""
                    for args in task['input_args']:
                        method_param += f"{args['name']}:{args['type']},"
                        generic_input_struct += f"""
pub {args['name']}:{args['type']},"""
                        field_assign += f"{args['name']},"
                        input_dict = {
                            "task_name": task_name,
                            "field": args['name']
                        }
                        input_fields.append(input_dict)
                        main_input_dict['init'] = input_fields

                    new_method = new_method_gen(
                        method_param, field_assign, task_name)
                    task_struct_impl += method_implementer(
                        task_name, new_method, setter_method, output_method)

            if key == "pipe":

                for item in values['map']:
                    input_dict = dict()
                    if item['task_name'] == task_name:
                        input_field_name = ""
                        output_field_name = ""
                        dependent_task = ""
                        for args in task['input_args']:
                            input_field_name += args['name']
                        for args in task['output_args']:
                            output_field_name += args['name']
                        for items in item['dependent_task']:
                            dependent_task += convert_to_pascalcase(
                                items['name'])
                        setter_method = setter_map(
                            dependent_task, input_field_name, output_field_name)
                        method_param = ""
                        field_assign = ""
                        new_method = new_method_gen("", "", task_name)
                        output_method = out_put_method("map", task_name)
                        task_struct_impl += method_implementer(
                            task_name, new_method, setter_method, output_method)
                        input_dict = {"task_name": task_name, "field": []}
                        dep_task.append(input_dict)
                        main_input_dict['map'] = dep_task

                for item in values['concat']:
                    input_dict = dict()
                    input_fields = []
                    if item['task_name'] == task_name:
                        task1 = ""
                        task2 = ""
                        field_name = ""
                        depend_task = ""
                        field_param = ""

                        input_dict = {"task_name": task_name, "field": []}
                        task1 = item['dependent_task'][0]['name']
                        task2 = item['dependent_task'][1]['name']
                        for args in task['input_args']:
                            field_name += args['name']
                        setter_method = setter_concat(task1, task2, field_name)
                        new_method = new_method_gen("", "", task_name)
                        output_method = out_put_method("", task_name)
                        task_struct_impl += method_implementer(
                            task_name, new_method, setter_method, output_method)
                        input_fields.append(input_dict)
                        main_input_dict['concat'] = input_fields
            if key == "pipe":

                for item in values['no_op']:
                    input_fields = []
                    input_dict = dict()
                    
                    if item['task_name'] == task_name:
                        method_param = ""
                        field_assign = ""
                        depend_task = ""
                        field_param = ""
                        for dep in item['dependent_task']:
                            depend_task = convert_to_pascalcase(dep['name'])
                            field_param = "".join(dep['fields'])

                        for args in task['input_args']:
                            if field_param != args['name']:
                                method_param += f"{args['name']}:{args['type']},"
                                generic_input_struct += f"""
pub {args['name']}:{args['type']},"""
                                field_assign += f"{args['name']},"
                                input_fields.append(args['name'])

                        input_dict = {"task_name": task_name,
                                      "field": input_fields}
                        setter_method = setter_no_op(depend_task, field_param)
                        output_method = out_put_method("", task_name)

                        new_method = new_method_gen(
                            method_param, field_assign, task_name)

                        task_struct_impl += method_implementer(
                            task_name, new_method, setter_method, output_method)
                        dep_task.append(input_dict)
                        main_input_dict['pipe'] = dep_task
            if key == "term":
                dependen_task = []
                for item in values['no_op']:
                    input_fields = []
                    input_dict = dict()
                    if item['task_name'] == task_name:
                        method_param = ""
                        field_assign = ""
                        depend_task = ""
                        field_param = ""
                        local_term_list = []
                        for dep in item['dependent_task']:
                            depend_task = convert_to_pascalcase(dep['name'])
                            field_param = "".join(dep['fields'])

                        for args in task['input_args']:
                            if field_param != args['name']:
                                method_param += f"{args['name']}:{args['type']},"
                                generic_input_struct += f"""
pub {args['name']}:{args['type']},"""
                                field_assign += f"{args['name']},"
                                local_term_list.append(args['name'])
                                input_fields.append(args['name'])

                        input_dict = {"task_name": task_name,
                                      "field": input_fields}

                        setter_method = setter_no_op(depend_task, field_param)
                        output_method = out_put_method("", task_name)

                        new_method = new_method_gen(
                            method_param, field_assign, task_name)

                        task_struct_impl += method_implementer(
                            task_name, new_method, setter_method, output_method)
                        dependen_task.append(input_dict)
                        main_input_dict['term'] = dependen_task
    task_struct_impl += creat_genric_input(generic_input_struct)
    return



"""
    Creates main function to use and run workflow generated from yaml config
"""


def create_main_function(tasks):
    global main_input_dict
    global dependencies
    global task_store, task_store_copy, dependency_matrix, task_struct_impl, main_file
    main = ""
    flow = ""
    initilization = ""
    workflow = ""
    workflow_edges = ""
    result = ""
    flow_final= ""
    final_initilization = ""
    final_destination =""
    for task in tasks:
        task_name = convert_to_pascalcase(task['task_name'])
        for key, values in main_input_dict.items():
            final_initilization = ""
            if key == "init":
                field  =""
                for value in values:
                    if task_name == value['task_name']:
                        field += f"input.{value['field']},"
                if task_name == value['task_name']:       
                    initilization += create_initialization_object(task['task_name'],field)
                    flow += f"""
let {value['task_name'].lower()}_index = workflow.add_node(Box::new({value['task_name'].lower()}));"""
                final_initilization += initilization
                initilization = ""
            
            elif key == "pipe":
                for value in values:
                    if task_name == value['task_name']:
                        if value['field'] == []:
                            flow += create_flow_objects(value)
                            initilization += create_initialization_object(
                                task['task_name'], "")
                        else:
                            fields = ""
                            for filed_value in value['field']:
                                fields += f"input.{filed_value},"
                            flow += create_flow_objects(value)
                            initilization += create_initialization_object(
                                task['task_name'], fields)
                        final_initilization += initilization
                        initilization =""
                
            elif key == "term":
                for value in values:
                    fields =""
                    if task_name == value['task_name']:
                        if value['field'] == []:
                            flow += create_flow_objects(value)
                            pass
                        else:
                            for filed_value in value['field']:
                                fields += f"input.{filed_value},"
                            flow += create_flow_objects(value)
                        initilization += create_initialization_object(
                                task['task_name'], fields)
                        final_initilization += initilization
                        initilization =""
            if final_initilization != "":
                final_destination += final_initilization
        flow_final += flow
        flow = ""

        for key, values in dependencies.items():
            if key == "init":
                for items in values['no_op']:
                    if task_name == items['task_name']:
                        workflow += f""" workflow.init()?"""
            if key == "pipe":
                for items in values['map']:
                    if task_name == items['task_name']:
                        dependent_field = ""
                        for dep_task in items['dependent_task']:
                            dependent_field += convert_to_pascalcase(
                                dep_task['name']).lower()
                        workflow += f""".pipe({items['task_name'].lower()}_index)?"""
                        workflow_edges += f"""
({dependent_field}_index,{items['task_name'].lower()}_index),"""
                for items in values['concat']:
                    if task_name == items['task_name']:
                        workflow += f""".pipe({items['task_name'].lower()}_index)?"""
                        dependent_field = ""
                        for dep_task in items['dependent_task']:
                            dependent_field += convert_to_pascalcase(
                                dep_task['name']).lower()
                            workflow_edges += f"""
({dependent_field}_index,{items['task_name'].lower()}_index),"""
                            dependent_field = ""
                for items in values['no_op']:
                    if task_name == items['task_name']:
                        workflow += f""".pipe({items['task_name'].lower()}_index)?"""
                        dependent_field = ""
                        for dep_task in items['dependent_task']:
                            dependent_field += convert_to_pascalcase(
                                dep_task['name']).lower()
                        workflow_edges += f"""
({dependent_field}_index,{items['task_name'].lower()}_index),"""

            if key == "term":
                for items in values['no_op']:
                    if task_name == items['task_name']:
                        workflow += f""".term(Some({items['task_name'].lower()}_index))?"""
                        dependent_field = ""
                        for dep_task in items['dependent_task']:
                            dependent_field += convert_to_pascalcase(
                                dep_task['name']).lower()
                        workflow_edges += f"""
({dependent_field}_index,{items['task_name'].lower()}_index),"""
                        result += f"""
let result: {items['task_name']}Output = result.try_into().unwrap();
let result = serde_json::to_value(result).unwrap();
Ok(result)
"""
    task_copy = tasks
    if "term" not in workflow:
        workflow += f".term(None)?"
        result += f"""
let result: {convert_to_pascalcase(task_copy[len(task_copy)-1]['task_name'])}Output = result.try_into().unwrap();
let result = serde_json::to_value(result).unwrap();
Ok(result)
"""

    edges = f"""
 workflow.add_edges(&[

       {workflow_edges}
]);
"""
    global_imports = global_import_generator(tasks)
    main += f"""
    {global_imports}

    {run_function}
    
    #[allow(dead_code, unused)]
    pub fn main(args:Value) -> Result<Value,String>{{
    const LIMIT : usize = {len(tasks)} ;   
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;
    {final_destination}
    {flow_final}
    {edges}
    let result = {workflow};
    {result}
    
}}

#[no_mangle]
pub unsafe extern "C" fn memory_alloc(size: u32, alignment: u32) -> *mut u8 {{
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}}

#[no_mangle]
pub unsafe extern "C" fn free_memory(ptr: *mut u8, size: u32, alignment: u32) {{
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}}

#[link(wasm_import_module = "host")]
extern "C" {{
    fn set_output(ptr: i32, size: i32);
}}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {{
    pub result: Value,
}}

    """
    main_file += main
    return


"""
    To Write generated code to Rust package 
"""


def generate_output(workflow_config: str, task_list):
    global common_rs_file, traits_file, task_struct_impl, main_file
    
    cargo_dependency = cargo_generator(task_list)
    workflow_config += cargo_dependency

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
    return
