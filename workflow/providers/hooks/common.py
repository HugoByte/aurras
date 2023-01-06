

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
        initialization = f"""
let {convert_to_pascalcase(task_name).lower()} = {convert_to_pascalcase(task_name)}::new({fields}String::from("{task_name}"));
"""
        return initialization
    else:
        initialization = f"""
let {convert_to_pascalcase(task_name).lower()}= {convert_to_pascalcase(task_name)}::new(String::from("{task_name}"));
"""
        return initialization


"""
    Generates Main Input Struct for the workflow exectution
"""


def create_generic_input(input_struct_field) -> str:

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
fn output(&self) ->Value{{
    self.mapout.clone()
}}
"""
    else:
        return f"""
fn output(&self) ->Value{{
    self.output.clone()
}}
"""


"""
    Generate setter method for the task struct which is not bounded by any operator
"""


def setter_no_op(depend_task, field) -> str:
    setter = f"""
fn setter(&mut self, value: Value) {{
        let value = value.get("{field}").unwrap();
        self.input.{field} = serde_json::from_value(value.clone()).unwrap();
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


def setter_concat(field) -> str:
    setter = f"""
fn setter(&mut self, value: Value) {{

        let value: Vec<Value> = serde_json::from_value(value).unwrap();

        let res = join_hashmap(
            serde_json::from_value(value[0].to_owned()).unwrap(),
            serde_json::from_value(value[1].to_owned()).unwrap(),
        );

        self.input.{field} = res;
}} 
"""
    return setter


"""
    Generate setter method for the task struct which is bounded by Map operator
"""


def setter_map(dep_task, input_field, output_field, dep_task_field_name, input_type) -> str:

    setter = f"""
fn setter(&mut self, value: Value) {{
        let value = value.get("{dep_task_field_name}").unwrap();
        let value = serde_json::from_value::<Vec<{input_type}>>(value.clone()).unwrap();
        let mut map: HashMap<_, _> = value
            .iter()
            .map(|x| {{
                self.input.{input_field} = x.to_owned();
                self.run();
                (x.to_owned(),
                self.output.get("{output_field}").unwrap().to_owned(),)

            }})
            .collect();
         self.mapout = to_value(map).unwrap();
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
