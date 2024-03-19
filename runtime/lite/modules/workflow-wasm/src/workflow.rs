
use std::collections::HashMap;
use std::fs; // for file system access
use wasm_bindgen::prelude::*;

// Define an enum to represent different actions (change this based on your actual actions)
#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Action {
    Polkadot,
    OpenWhisk,
    HelloWorld,
}

// Trait representing a workflow. This allows for flexibility in defining workflow behavior.
pub trait Workflow {
    fn get_name(&self) -> &str;
    fn get_wasm_path(&self) -> &str; // Path to the WASM file
    fn get_wasm_code(&self) -> Result<Vec<u8>, String> {
        // Load WASM code from the path
        fs::read(self.get_wasm_path()).map_err(|err| format!("Error loading WASM: {}", err))
    }
    fn handle_event(&self, event: &str) -> Result<String, String>;
}

// Define concrete workflow structs implementing the Workflow trait
#[derive(Clone)]
pub struct PolkadotWorkflow {
    pub wasm_path: String,
}

impl Workflow for PolkadotWorkflow {
    fn get_name(&self) -> &str {
        "polkadot"
    }

    fn get_wasm_path(&self) -> &str {
        &self.wasm_path
    }

    fn handle_event(&self, event: &str) -> Result<String, String> {
        Ok(format!("Handling polkadot event: {}", event))
    }
}

#[derive(Clone)]
pub struct OpenWhiskWorkflow {
    pub wasm_path: String,
}

impl Workflow for OpenWhiskWorkflow {
    fn get_name(&self) -> &str {
        "openwhisk"
    }

    fn get_wasm_path(&self) -> &str {
        &self.wasm_path
    }

    fn handle_event(&self, event: &str) -> Result<String, String> {
        Ok(format!("Handling openwhisk event: {}", event))
    }
}


// Function to store workflow with its implementation
pub fn store_workflow(workflow: Box<dyn Workflow>) -> Result<(), String> {
    // store workflow information
    println!("Storing workflow: {}", workflow.get_name());
    Ok(())
}

// HashMap to store workflows
static mut WORKFLOW_MAP: Option<HashMap<String, Box<dyn Workflow>>> = None;

pub fn get_workflow_map() -> &'static mut HashMap<String, Box<dyn Workflow>> {
    unsafe {
        WORKFLOW_MAP.get_or_insert(HashMap::new())
    }
}

pub fn handle_event(event_name: &str, action: Action) -> Result<String, String> {
    let workflow_map = get_workflow_map();
    let workflow = workflow_map.get(event_name);

    match workflow {
        Some(workflow) => {
            match action {
                Action::Polkadot => {
                    let wasm_code = workflow.get_wasm_code()?;
                    println!("Loaded WASM code for {} with size: {}", workflow.get_name(), wasm_code.len());
                    workflow.handle_event(event_name)
                },
                Action::OpenWhisk => {
                    let wasm_code = workflow.get_wasm_code()?;
                    println!("Loaded WASM code for {} with size: {}", workflow.get_name(), wasm_code.len());
                    workflow.handle_event(event_name)
                },
                Action::HelloWorld => {
                    // Handle HelloWorld event here (assuming your workflow handles it)
                    // You can call workflow.handle_event(event_name) or implement your specific logic
                    Ok(format!("Handling HelloWorld event: {}", event_name))
                },
            }
        }
        None => Err(format!("Event '{}' not found in workflow map", event_name)),
    }
}
