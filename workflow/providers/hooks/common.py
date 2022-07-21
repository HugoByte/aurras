


"""
    Creates the flow objects for the workflow execution
"""

def create_flow_objects(value) -> str:

    flow_object = f"""
let {value['task_name'].lower()}_index = workflow.add_node(Box::new({value['task_name'].lower()}));"""

    return flow_object

"""
    Creates the initialization objects for the workflow initialization
"""

def create_initialization_object(task_name, fields) -> str:
    
    if fields != "":
        initializattion = f"""
let {convert_to_pascalcase(task_name).lower()} = {convert_to_pascalcase(task_name)}::new({fields}String::from("{task_name}"));
"""
        return initializattion
    else:
        initializattion = f"""
let {convert_to_pascalcase(task_name).lower()}= {convert_to_pascalcase(task_name)}::new(String::from("{task_name}"));
"""
        return initializattion


"""
    Generates Main Input Struct for the workflow exectution
"""

def creat_genric_input(input_struct_field) -> str:

    input_struct = f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Input {{
    {input_struct_field}
    
}}
"""

    return input_struct

"""
    Generate output method for the task struct
"""

def out_put_method(type, task_name) -> str:
    if type == "map":
        return f"""
fn output(&self) ->Mapout{task_name}{{
    self.mapout.clone()
}}
"""
    else:
        return f"""
fn output(&self) ->{task_name}Output{{
    self.output.clone()
}}
"""

"""
    Generate setter method for the task struct which is not bounded by any operator
"""

def setter_no_op(depend_task, field) -> str:
    setter = f"""
fn setter(&mut self, value: Types) {{
        let value: {depend_task}Output = value.try_into().unwrap();
        self.input.{field} = value.{field};
}}   
"""
    return setter


def new_method_gen(method_param, field_assign, task_name) -> str:
    if method_param != "" and field_assign != "":
        new_method_str = f"""
pub fn new({method_param}action_name:String) -> Self {{ Self{{  input:{task_name}Input{{{field_assign} ..Default::default()}},action_name: action_name, ..Default::default()}}}}
"""
        return new_method_str
    else:
        new_method_str = f"""
pub fn new(action_name:String) -> Self {{ Self{{  input:{task_name}Input{{..Default::default()}},action_name: action_name, ..Default::default()}}}}
"""
        return new_method_str

"""
    Generate setter method for the task struct which is bounded by concat operator
"""

def setter_concat(task1, task2, field) -> str:
    setter = f"""
fn setter(&mut self, value: Types) {{
        let value: Vec<Types> = value.try_into().unwrap();
        let value: (Mapout{convert_to_pascalcase(task1)}, Mapout{convert_to_pascalcase(task2)}) = (
            value[0].clone().try_into().unwrap(),
            value[1].clone().try_into().unwrap(),
        );
        

        
        let res = join_hashmap(value.0.result, value.1.result);
        
        self.input.{field} = res;
}} 
"""
    return setter

"""
    Generate setter method for the task struct which is bounded by Map operator
"""

def setter_map(dep_task, input_field, output_field) -> str:

    setter = f"""
fn setter(&mut self, value: Types) {{
        let value: {dep_task}Output = value.try_into().unwrap();
        let mut map: HashMap<_, _> = value
            .ids
            .iter()
            .map(|x| {{
                self.input.{input_field} = x.to_owned();
                self.run();
                (x.to_owned(), self.output.{output_field}.to_owned())
            }})
            .collect();
        self.mapout.result = map;
    }}

"""
    return setter

"""
    Implements methods for the task struct
        # Arguments
        `task_name`      -   Name of the task
        `new_method`     -   new method implementation string
        `setter_method`  -   setter method implementation string
        `output_method`  -   output method implementation string
"""

def method_implementer(task_name, new_method, setter_method, output_method) -> str:
    new_impl = ""
    new_impl += f"""
impl {task_name} {{

{new_method}
{setter_method}
{output_method}

}}
"""
    return new_impl

# to convert camel_case to PascalCase


def convert_to_pascalcase(string: str) -> str:

    return string.replace("_", " ").title().replace(" ", "")

