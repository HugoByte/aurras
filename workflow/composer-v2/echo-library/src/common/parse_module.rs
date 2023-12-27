use super::*;

/// Validates the kind name of the task and returns the formatted kind if valid
///
/// # Arguments
///
/// * `kind` - A reference to the kind name of the task
///
/// # Returns
///
/// * An Ok Result containing the formatted kind if the input is valid
/// * An Err Result with an ErrorKind::NotFound if the input is not valid
///
pub fn get_task_kind(kind: &str) -> Result<String, ErrorKind> {
    match kind.to_lowercase().as_str() {
        "openwhisk" => Ok("OpenWhisk".to_string()),
        "polkadot" => Ok("Polkadot".to_string()),
        "hello_world" => Ok("HelloWorldDerive".to_string()),
        _ => Err(ErrorKind::NotFound),
    }
}

fn get_main_method_code_template(tasks_length: usize) -> String {
    format!(
        "#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {{
    const LIMIT: usize = {tasks_length};
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;
"
    )
}

/// Formats the attributes from the given HashMap into a specific string format
/// This string will be passed to the macros as arguments
///
/// # Arguments
///
/// * `map` - A reference to the HashMap containing attribute key-value pairs
///
/// # Returns
///
/// * A String containing formatted attribute key-value pairs enclosed in square brackets
///
/// This formats the value of the attributes as enclosed by double quots
pub fn get_attributes(attributes: &HashMap<String, String>) -> String {
    let mut build_string = Vec::new();

    for (key, value) in attributes {
        build_string.push(format!("{}:\"{}\"", key.to_case(Case::Pascal), value));
    }

    format!("[{}]", build_string.join(","))
}

fn get_default_value_functions_code(workflow: &Workflow) -> String {
    let mut default_value_functions = String::new();

    for task in workflow.tasks.values() {
        for input in task.input_arguments.iter() {
            if !input.is_depend {
                if let Some(val) = input.default_value.as_ref() {
                    let content = match input.input_type {
                        RustType::String => format!("{val:?}.to_string()"),
                        _ => format!(
                            "let val = serde_json::from_str::<{}>({:?}).unwrap();val",
                            input.input_type, val
                        ),
                    };

                    let make_fn = format!(
                        "pub fn {}_fn() -> {}{{{}}}\n",
                        input.name, input.input_type, content
                    );

                    default_value_functions.push_str(&make_fn);
                }
            };
        }
    }

    default_value_functions
}

/// Creates a Rust code to generate a struct with fields representing inputs not
/// depending on any task
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing Rust code to create a struct representing inputs not depending
///   on any task
///
fn get_task_common_input_type_constructor(
    composer_custom_types: &HashMap<String, String>,
    workflow: &Workflow,
) -> Result<String, Error> {
    let mut common = Vec::<String>::new();
    let mut workflow_custom_types = Vec::<String>::new();

    for task in workflow.tasks.values() {
        for input in task.input_arguments.iter() {
            if let RustType::Struct(name) = &input.input_type {
                workflow_custom_types.push(name.to_string());
            }

            if !input.is_depend {
                if input.default_value.as_ref().is_some() {
                    common.push(format!(
                        "#[\"{}_fn\"] {}:{}",
                        input.name, input.name, input.input_type
                    ));
                } else {
                    common.push(format!("{}:{}", input.name, input.input_type));
                };
            }
        }
    }

    let workflow_custom_types = if !workflow_custom_types.is_empty() {
        let mut build_string = String::new();

        for custom_type in workflow_custom_types.iter() {
            let typ = match composer_custom_types.get(custom_type) {
                Some(t) => t,
                None => return Err(Error::msg("Missing custom type in workflow")),
            };
            build_string = format!("{build_string}{typ}");
        }

        build_string
    } else {
        "".to_string()
    };
    Ok(format!(
        "{workflow_custom_types}
make_input_struct!(
Input,
[{}],
[Debug, Clone, Default, Serialize, Deserialize]
);",
        common.join(",")
    ))
}

fn get_task_type_constructors(workflow: &Workflow) -> String {
    let mut constructors = String::new();

    for task in workflow.tasks.values() {
        let mut parameters = String::new();

        for argument in task.input_arguments.iter() {
            if !argument.is_depend {
                parameters.push_str(&format!("input.{},", argument.name));
            }
        }

        let constructor = format!(
            "let {} = {}::new({}\"{}\".to_string());\n",
            task.action_name.to_case(Case::Snake),
            task.action_name.to_case(Case::Pascal),
            parameters,
            task.action_name.clone()
        );

        constructors.push_str(&constructor);
    }

    constructors
}

fn get_task_input_type_constructors(workflow: &Workflow) -> String {
    let mut input_type_build_string = String::new();

    for task in workflow.tasks.values() {
        let mut arguments = Vec::new();

        for field in task.input_arguments.iter() {
            arguments.push(format!("{}:{}", field.name, field.input_type));
        }

        input_type_build_string.push_str(&format!(
            "make_input_struct!(\n{}Input,\n[{}],\n[Debug, Clone, Default, Serialize, Deserialize]\n);",
            task.action_name.to_case(Case::Pascal),
            arguments.join(",")
        ));
    }

    input_type_build_string
}

fn get_independent_fields(task: &Task) -> Vec<String> {
    let mut independent_fields = Vec::<String>::new();

    for field in task.input_arguments.iter() {
        if !field.is_depend {
            independent_fields.push(format!("{}:{}", field.name, field.input_type));
        }
    }

    independent_fields
}

/// Generates Rust code to create structs for each task and its input, and creates object
/// for these types inside the main function
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * An array of Strings containing Rust code to create structs and objects for the
///   specified workflow
///
fn get_task_main_type_constructors(workflow: &Workflow) -> Result<String, Error> {
    let mut input_structs = String::new();

    for (task_name, task) in workflow.tasks.iter() {
        let task_name = task_name.to_case(Case::Pascal);

        let independent_fields = get_independent_fields(task);

        let output_field = if task.operation.is_map() {
            "mapout"
        } else {
            "output"
        };

        input_structs = format!(
            "{input_structs}
make_main_struct!(
    {task_name},
    {task_name}Input,
    [Debug, Clone, Default, Serialize, Deserialize, {}],
    {},
    {}
);
impl_new!(
    {task_name},
    {task_name}Input,
    [{}]
);
",
            get_task_kind(&task.kind).unwrap(),
            get_attributes(&task.attributes),
            output_field,
            independent_fields.join(",")
        );
    }

    Ok(input_structs)
}

fn get_impl_setters_code(workflow: &Workflow) -> Result<String, Error> {
    let mut impl_setters_code = String::new();

    for (task_name, task) in workflow.tasks.iter() {
        let task_name = task_name.to_case(Case::Pascal);

        let mut setter_fields = Vec::<String>::new();

        let mut set = HashMap::<String, i32>::new();
        let mut index: i32 = 0;

        for dependent in task.depend_on.iter() {
            let current_index = if let Some(current_index) = set.get(&dependent.task_name) {
                index -= 1;
                current_index
            } else {
                set.insert(dependent.task_name.to_string(), index);
                &index
            };

            if task.operation.is_combine() {
                let dependent_task = match workflow.tasks.get(&dependent.task_name) {
                    Some(t) => t,
                    None => return Err(Error::msg("Missing custom type in workflow")),
                };

                if dependent_task.operation.is_map() {
                    setter_fields.push(format!(
                        "(value)[{}]{}:\"{}\"",
                        current_index, dependent.cur_field, dependent.prev_field
                    ));
                } else {
                    setter_fields.push(format!(
                        "[{}]{}:\"{}\"",
                        current_index, dependent.cur_field, dependent.prev_field
                    ));
                }
            } else {
                setter_fields.push(format!(
                    "{}:\"{}\"",
                    dependent.cur_field, dependent.prev_field
                ));
            }

            index += 1;
        }

        let setter_build_string = match &task.operation {
            Operation::Map(field) => format!(
                "impl_map_setter!({}, {}, {}, \"{}\");\n",
                task_name,
                setter_fields.join(","),
                task.input_arguments[0].input_type,
                field
            ),
            Operation::Concat => format!(
                "impl_concat_setter!({}, {});\n",
                task_name, task.input_arguments[0].name
            ),
            Operation::Combine => format!(
                "impl_combine_setter!({},[{}]);\n",
                task_name,
                setter_fields.join(","),
            ),
            _ => format!(
                "impl_setter!({}, [{}]);\n",
                task_name,
                setter_fields.join(",")
            ),
        };

        impl_setters_code.push_str(&setter_build_string);
    }

    Ok(impl_setters_code)
}

/// Generates Rust code to call the `impl_execute_trait!` macro with the arguments as all
/// of the task names
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing the Rust code to call the `impl_execute_trait!` macro
///
fn get_impl_execute_trait_code(workflow: &Workflow) -> String {
    let mut task_names = Vec::new();

    for task_name in workflow.tasks.keys() {
        task_names.push(task_name.to_case(Case::Pascal));
    }

    format!("impl_execute_trait!({});", task_names.join(","))
}

fn get_add_nodes_code(flow: &Vec<String>) -> String {
    let mut add_nodes_code = String::new();

    for i in 0..flow.len() {
        add_nodes_code.push_str(&format!(
            "let {}_index = workflow.add_node(Box::new({}));\n",
            flow[i].to_case(Case::Snake),
            flow[i].to_case(Case::Snake)
        ));
    }

    add_nodes_code
}

fn get_add_edges_code(workflow: &Workflow, flow: &Vec<String>) -> Result<String, Error> {
    let mut add_edges_code = "workflow.add_edges(&[\n".to_string();

    for index in 0..flow.len() - 1 {
        if let Some(dependent_task) = workflow.tasks.get(&flow[index + 1]) {
            let mut set = HashSet::<String>::new();

            for dependent_task in dependent_task.depend_on.iter() {
                if !set.contains(&dependent_task.task_name) {
                    add_edges_code = format!(
                        "{add_edges_code}({}_index, {}_index),\n",
                        dependent_task.task_name.to_case(Case::Snake),
                        flow[index + 1].to_case(Case::Snake)
                    );
                    set.insert(dependent_task.task_name.clone());
                }
            }
        } else {
            return Err(Error::msg(" Error adding the edges "));
        }
    }

    add_edges_code += "]);";
    Ok(add_edges_code)
}

fn get_add_execute_workflow_code(workflow: &Workflow, flow: &Vec<String>) -> Result<String, Error> {
    let mut execute_code = "let result = workflow\n.init()?".to_string();

    for task_index in 0..flow.len() - 1 {
        execute_code = if task_index + 1 == flow.len() - 1 {
            match workflow
                .tasks
                .get(&flow[task_index + 1])
                .unwrap()
                .depend_on
                .len()
            {
                0 | 1 => {
                    format!(
                        "{execute_code}\n.term(Some({}_index))?;",
                        flow[task_index + 1].to_case(Case::Snake)
                    )
                }

                _ => {
                    format!(
                        "{execute_code}\n.pipe({}_index)?\n.term(None)?;",
                        flow[task_index + 1].to_case(Case::Snake)
                    )
                }
            }
        } else {
            format!(
                "{execute_code}\n.pipe({}_index)?",
                flow[task_index + 1].to_case(Case::Snake)
            )
        }
    }

    Ok(execute_code)
}

/// Generates Rust code to add workflow nodes and edges
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * An array containing the Rust code to add workflow nodes and edges
///
fn get_workflow_nodes_and_edges_code(workflow: &Workflow) -> Result<String, Error> {
    let flow: Vec<String> = workflow.get_flow();

    if flow.is_empty() {
        return Ok("".to_string());
    }

    if flow.len() == 1 {
        return Ok(format!(
            "\
let {}_index = workflow.add_node(Box::new({}));
\tlet result = workflow\n\t\t.init()?
\t\t.term(None)?;
Ok(result)
",
            flow[0].to_case(Case::Snake),
            flow[0].to_case(Case::Snake)
        ));
    }

    Ok(format!(
        "{}\n{}\n{}let result = serde_json::to_value(result).unwrap();\nOk(result)",
        get_add_nodes_code(&flow),
        get_add_edges_code(workflow, &flow)?,
        get_add_execute_workflow_code(workflow, &flow)?,
    ))
}

/// Generates the main Rust code for the workflow package and creates the `types.rs` file
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing the Rust code to be written to `types.rs` file in the workflow package
///
pub fn generate_types_rs_file_code(
    workflow: &Workflow,
    custom_types: &HashMap<String, String>,
) -> Result<String, Error> {
    let main_file = format!(
        "use super::*;\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}}}",
        add_polkadot_openwhisk(workflow),
        get_task_input_type_constructors(workflow),
        get_task_main_type_constructors(workflow)?,
        get_impl_setters_code(workflow)?,
        get_default_value_functions_code(workflow),
        get_task_common_input_type_constructor(custom_types, workflow)?,
        get_impl_execute_trait_code(workflow),
        get_main_method_code_template(workflow.tasks.len()),
        get_task_type_constructors(workflow),
        get_workflow_nodes_and_edges_code(workflow)?
    );
    Ok(main_file)
}

fn get_openwhisk_kind_dependencies() -> String {
    "
openwhisk_macro = \"0.1.6\"

"
    .to_string()
}

fn get_polkadot_kind_dependencies() -> String {
    // some of the polkadot dependencies
    "substrate_macro = \"0.1.3\"
    pallet-staking = { git = \"https://github.com/paritytech/substrate.git\", package = \"pallet-staking\", rev = \"eb1a2a8\" }
    substrate-api-client = { git = \"https://github.com/HugoByte/substrate-api-client.git\", default-features = false, features = [\"staking-xt\"], branch =\"wasm-support\"}
sp-core = { version = \"6.0.0\", default-features = false, features = [\"full_crypto\"], git = \"https://github.com/paritytech/substrate.git\", rev = \"eb1a2a8\" }
sp-runtime = { version = \"6.0.0\", default-features = false, git = \"https://github.com/paritytech/substrate.git\", rev = \"eb1a2a8\" }
     "
        .to_string()
}

pub fn generate_cargo_toml_dependencies(workflow: &Workflow) -> String {
    let mut dependency_map = HashMap::new();

    let hello_world_dependency = "hello_world_macro = {git= \"https://github.com/shanithkk/aurras\", branch = \"feature/composer-v2\", package = \"hello_world_macro\"}"
    .to_string();

    dependency_map.insert("hello_world", hello_world_dependency);
    dependency_map.insert("openwhisk", get_openwhisk_kind_dependencies());
    dependency_map.insert("polkadot", get_polkadot_kind_dependencies());

    let kinds = get_common_kind(workflow);
    if kinds.is_empty() {
        return String::new();
    }

    let mut toml_dependencies = String::new();
    for (kind, dependency_string) in dependency_map.iter() {
        if kinds.contains(&kind.to_string()) {
            toml_dependencies.push_str(dependency_string);
        }
    }

    toml_dependencies
}

pub fn handle_multiple_dependency() -> String {
    let openwhisk_dependency = get_openwhisk_kind_dependencies();
    let polkadot_dependency = get_polkadot_kind_dependencies();

    let combined_dependencies = format!("{}{}", openwhisk_dependency, polkadot_dependency);
    combined_dependencies
}

pub fn get_polkadot() -> String {
    "\
    use substrate_macro::Polkadot;
    use sp_core::H256;

    "
    .to_string()
}

pub fn get_openwhisk() -> String {
    "\
    use openwhisk_macro::*;
    use openwhisk_rust::*;
    
    "
    .to_string()
}

pub fn add_polkadot_openwhisk(workflow: &Workflow) -> String {
    let kinds = get_common_kind(workflow);

    let mut toml_dependencies = String::new();

    if kinds.contains("openwhisk") {
        toml_dependencies = format!("{}", get_openwhisk());
    }

    if kinds.contains("polkadot") {
        toml_dependencies = format!("{}", get_polkadot());
    }

    if kinds.contains("openwhisk") && kinds.contains("polkadot") {
        toml_dependencies = handle_multiple_kinds();
    }

    if kinds.contains("hello_world") {
        toml_dependencies += "\nuse hello_world_macro::HelloWorldDerive;";
    }

    toml_dependencies
}

pub fn staking_ledger() -> String {
    "\
use sp_runtime::AccountId32;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
 pub struct StakingLedger {
 pub stash: AccountId32,
 #[codec(compact)]
 pub total: u128,
 #[codec(compact)]
 pub active: u128,
 pub unlocking: Vec<u32>,
 pub claimed_rewards: Vec<u32>,
}
    "
    .to_string()
}

pub fn get_struct_stake_ledger(workflow: &Workflow) -> String {
    let kinds = get_common_kind(workflow);

    let mut toml_dependencies = String::new();

    if kinds.contains("polkadot") {
        toml_dependencies = format!("{}", staking_ledger());
    }

    toml_dependencies
}

pub fn get_common_kind(workflow: &Workflow) -> HashSet<String> {
    let mut kinds = HashSet::new();
    for task in workflow.tasks.values() {
        kinds.insert(task.kind.to_lowercase());
    }
    kinds
}

pub fn handle_multiple_kinds() -> String {
    let openwhisk = get_openwhisk();
    let polkadot = get_polkadot();

    let combined_dependencies = format!("{}{}", openwhisk, polkadot);
    combined_dependencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_main_method_code_template() {
        let output = get_main_method_code_template(4);

        assert_eq!(
            &output,
            "#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;
"
        );
    }

    #[test]
    fn test_get_attributes() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());

        let output = get_attributes(&attributes);
        assert_eq!(output, "[Key:\"value\"]");
    }

    #[test]
    fn test_get_default_value_functions_code() {
        let task1 = Task {
            action_name: "task0".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::String,
                    default_value: Some("value_x".to_string()),
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::List(Box::new(RustType::String)),
                    default_value: Some("[\"val1,\"val2\"]".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task1".to_string(), task1);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_default_value_functions_code(&workflow);

        assert_eq!(
        output,
        "\
pub fn argument_1_fn() -> String{\"value_x\".to_string()}
pub fn argument_2_fn() -> Vec<String>{let val = serde_json::from_str::<Vec<String>>(\"[\\\"val1,\\\"val2\\\"]\").unwrap();val}
"
    )
    }

    #[test]
    fn test_get_task_common_input_type_constructor() {
        let task0 = Task {
            action_name: "task0".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    is_depend: true,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
                Input {
                    name: "argument_3".to_string(),
                    input_type: RustType::List(Box::new(RustType::Uint)),
                    ..Default::default()
                },
                Input {
                    name: "argument_4".to_string(),
                    input_type: RustType::Float,
                    is_depend: true,
                    ..Default::default()
                },
                Input {
                    name: "argument_5".to_string(),
                    input_type: RustType::String,
                    ..Default::default()
                },
                Input {
                    name: "argument_6".to_string(),
                    input_type: RustType::HashMap(
                        Box::new(RustType::Int),
                        Box::new(RustType::Float),
                    ),
                    ..Default::default()
                },
                Input {
                    name: "argument_7".to_string(),
                    input_type: RustType::Tuple(Box::new(RustType::Int), Box::new(RustType::Float)),
                    ..Default::default()
                },
                Input {
                    name: "argument_8".to_string(),
                    input_type: RustType::Struct("Struct1".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let mut custom_types = HashMap::new();

        custom_types.insert(
        "Struct1".to_string(),
        "make_input_struct!(\nStruct1,\n{field1:i32},\n[Default, Clone, Debug, Deserialize, Serialize]\n);".to_string());

        let output = get_task_common_input_type_constructor(&custom_types, &workflow);
        assert_eq!(
        &output.unwrap(),
        "\
make_input_struct!(
Struct1,
{field1:i32},
[Default, Clone, Debug, Deserialize, Serialize]
);
make_input_struct!(
Input,
[argument_2:i32,argument_3:Vec<u32>,argument_5:String,argument_6:HashMap<i32,f32>,argument_7:(i32,f32),argument_8:Struct1],
[Debug, Clone, Default, Serialize, Deserialize]
);")
    }

    #[test]
    fn test_get_task_type_constructors() {
        let task0 = Task {
            action_name: "task0".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_task_type_constructors(&workflow);

        assert_eq!(
            output,
            "let task_0 = Task0::new(input.argument_1,input.argument_2,\"task0\".to_string());\n"
        );
    }

    #[test]
    fn test_get_task_input_type_constructors() {
        let task0 = Task {
            action_name: "task0".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_task_input_type_constructors(&workflow);

        println!("{:?}", output);

        assert_eq!(
            output,
            "make_input_struct!(
Task0Input,
[argument_1:bool,argument_2:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);"
        );
    }

    #[test]
    fn test_get_independent_fields() {
        let task0 = Task {
            action_name: "task0".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    is_depend: true,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let output = get_independent_fields(&task0);

        assert_eq!(output, vec!["argument_2:i32"]);
    }

    #[test]
    fn test_get_task_main_type_constructors() {
        let task0 = Task {
            action_name: "task0".to_string(),
            kind: "Openwhisk".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_task_main_type_constructors(&workflow);

        assert_eq!(
            output.unwrap(),
            "
make_main_struct!(
    Task0,
    Task0Input,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [],
    output
);
impl_new!(
    Task0,
    Task0Input,
    [argument_1:bool,argument_2:i32]
);
"
        );
    }

    #[test]
    fn test_get_impl_setters_code() {
        let task0 = Task {
            action_name: "task0".to_string(),
            kind: "Openwhisk".to_string(),
            input_arguments: vec![
                Input {
                    name: "argument_1".to_string(),
                    input_type: RustType::Boolean,
                    is_depend: true,
                    ..Default::default()
                },
                Input {
                    name: "argument_2".to_string(),
                    input_type: RustType::Int,
                    ..Default::default()
                },
            ],
            depend_on: vec![Depend {
                task_name: "task1".to_string(),
                cur_field: "argument_1".to_string(),
                prev_field: "data_field".to_string(),
            }],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_impl_setters_code(&workflow);

        assert_eq!(
            output.unwrap(),
            "impl_setter!(Task0, [argument_1:\"data_field\"]);\n"
        );
    }

    #[test]
    fn test_get_impl_execute_trait_code() {
        let task0 = Task {
            action_name: "task0".to_string(),
            kind: "Openwhisk".to_string(),
            ..Default::default()
        };

        let task1 = Task {
            action_name: "task1".to_string(),
            kind: "Openwhisk".to_string(),
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);
        tasks.insert("task1".to_string(), task1);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let output = get_impl_execute_trait_code(&workflow);
        assert!(
            output == "impl_execute_trait!(Task0,Task1);"
                || output == "impl_execute_trait!(Task1,Task0);"
        );
    }

    #[test]
    fn test_get_add_nodes_code() {
        let flow = vec![
            "task0".to_string(),
            "task2".to_string(),
            "task1".to_string(),
            "task4".to_string(),
            "task3".to_string(),
        ];

        let output = get_add_nodes_code(&flow);

        assert_eq!(
            output,
            "\
let task_0_index = workflow.add_node(Box::new(task_0));
let task_2_index = workflow.add_node(Box::new(task_2));
let task_1_index = workflow.add_node(Box::new(task_1));
let task_4_index = workflow.add_node(Box::new(task_4));
let task_3_index = workflow.add_node(Box::new(task_3));
"
        )
    }

    #[test]
    fn test_get_add_edges_code() {
        let task0 = Task {
            action_name: "task0".to_string(),
            ..Default::default()
        };
        let task1 = Task {
            action_name: "task1".to_string(),
            depend_on: vec![Depend {
                task_name: "task0".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let task2 = Task {
            action_name: "task2".to_string(),
            depend_on: vec![
                Depend {
                    task_name: "task1".to_string(),
                    ..Default::default()
                },
                Depend {
                    task_name: "task0".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let task3 = Task {
            action_name: "task3".to_string(),
            depend_on: vec![Depend {
                task_name: "task2".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let task4 = Task {
            action_name: "task4".to_string(),
            depend_on: vec![
                Depend {
                    task_name: "task3".to_string(),
                    ..Default::default()
                },
                Depend {
                    task_name: "task2".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);
        tasks.insert("task1".to_string(), task1);
        tasks.insert("task2".to_string(), task2);
        tasks.insert("task3".to_string(), task3);
        tasks.insert("task4".to_string(), task4);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let flow = workflow.get_flow();

        let output = get_add_edges_code(&workflow, &flow);

        assert_eq!(
            output.unwrap(),
            "\
workflow.add_edges(&[
(task_0_index, task_1_index),
(task_1_index, task_2_index),
(task_0_index, task_2_index),
(task_2_index, task_3_index),
(task_3_index, task_4_index),
(task_2_index, task_4_index),
]);"
        );
    }

    #[test]
    fn test_get_add_execute_workflow_code() {
        let task0 = Task {
            action_name: "task0".to_string(),
            ..Default::default()
        };
        let task1 = Task {
            action_name: "task1".to_string(),
            depend_on: vec![Depend {
                task_name: "task0".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let task2 = Task {
            action_name: "task2".to_string(),
            depend_on: vec![
                Depend {
                    task_name: "task1".to_string(),
                    ..Default::default()
                },
                Depend {
                    task_name: "task0".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let task3 = Task {
            action_name: "task3".to_string(),
            depend_on: vec![Depend {
                task_name: "task2".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let task4 = Task {
            action_name: "task4".to_string(),
            depend_on: vec![
                Depend {
                    task_name: "task3".to_string(),
                    ..Default::default()
                },
                Depend {
                    task_name: "task2".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);
        tasks.insert("task1".to_string(), task1);
        tasks.insert("task2".to_string(), task2);
        tasks.insert("task3".to_string(), task3);
        tasks.insert("task4".to_string(), task4);

        let workflow = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks,
        };

        let flow = workflow.get_flow();

        let output = get_add_execute_workflow_code(&workflow, &flow);

        assert_eq!(
            output.unwrap(),
            "\
let result = workflow
.init()?
.pipe(task_1_index)?
.pipe(task_2_index)?
.pipe(task_3_index)?
.pipe(task_4_index)?
.term(None)?;"
        );
    }
}
