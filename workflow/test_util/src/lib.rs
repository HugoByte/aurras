use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(test)]
mod wasi_http;

pub mod helper;
pub use helper::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    allowed_hosts: Option<Vec<String>>,
    data: Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::{
        fs,
        sync::{Arc, Mutex},
    };
    use wasi_common::WasiCtx;
    use wasi_http::HttpCtx;
    use wasmtime::Linker;
    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;

    #[allow(dead_code)]
    fn run_workflow(data: Value, path: String) -> Output {
        let wasm_file = fs::read(path).unwrap();
        let input: MainInput = serde_json::from_value(data).unwrap();
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        let output: Arc<Mutex<Output>> = Arc::new(Mutex::new(Output {
            result: serde_json::json!({}),
        }));
        let output_ = output.clone();
        let buf = serde_json::to_vec(&input).expect("should serialize");
        let mem_size: i32 = buf.len() as i32;

        linker
            .func_wrap("host", "get_input_size", move || -> i32 { mem_size })
            .expect("should define the function");

        linker
            .func_wrap(
                "host",
                "set_output",
                move |mut caller: Caller<'_, WasiCtx>, ptr: i32, capacity: i32| {
                    let output = output_.clone();
                    let mem = match caller.get_export("memory") {
                        Some(Extern::Memory(mem)) => mem,
                        _ => return Err(Trap::new("failed to find host memory")),
                    };
                    let offset = ptr as u32 as usize;
                    let mut buffer: Vec<u8> = vec![0; capacity as usize];
                    match mem.read(&caller, offset, &mut buffer) {
                        Ok(()) => {
                            println!(
                                "Buffer = {:?}, ptr = {}, capacity = {}",
                                buffer, ptr, capacity
                            );
                            match serde_json::from_slice::<Output>(&buffer) {
                                Ok(serialized_output) => {
                                    let mut output = output.lock().unwrap();
                                    *output = serialized_output;
                                    Ok(())
                                }
                                Err(err) => {
                                    let msg = format!("failed to serialize host memory: {}", err);
                                    Err(Trap::new(msg))
                                }
                            }
                        }
                        _ => Err(Trap::new("failed to read host memory")),
                    }
                },
            )
            .expect("should define the function");

        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()
            .unwrap()
            .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_binary(&engine, &wasm_file).unwrap();
        let max_concurrent_requests = Some(42);

        let http = HttpCtx::new(input.allowed_hosts, max_concurrent_requests).unwrap();
        http.add_to_linker(&mut linker).unwrap();

        let linking = linker.instantiate(&mut store, &module).unwrap();

        let malloc = linking
            .get_typed_func::<(i32, i32), i32, _>(&mut store, "memory_alloc")
            .unwrap();
        let data = serde_json::to_vec(&input.data).unwrap();
        let data_ptr = malloc.call(&mut store, (data.len() as i32, 2)).unwrap();

        let memory = linking.get_memory(&mut store, "memory").unwrap();
        memory.write(&mut store, data_ptr as usize, &data).unwrap();
        let len: i32 = data.len().try_into().unwrap();
        let run = linking
            .get_typed_func::<(i32, i32), (), _>(&mut store, "_start")
            .unwrap();
        let _result_from_wasm = run.call(&mut store, (data_ptr, len));
        let malloc = linking
            .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "free_memory")
            .unwrap();
        malloc
            .call(&mut store, (data_ptr, data.len() as i32, 2))
            .unwrap();

        let res = output.lock().unwrap().clone();
        res
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Output {
        pub result: Value,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Resultss {
        result: String,
    }

    #[async_std::test]
    async fn test_car_market_place() {
        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../examples/car_market_place_mock.wasm".to_string());
        let server = post("127.0.0.1:8080").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "car_type":"hatchback",
                "company_name":"maruthi",
                "model_name":"alto",
                "price":1200000
                }
        });
        let result = run_workflow(input, path);

        assert!(result
            .result
            .to_string()
            .contains("Thank you for the purchase"))
    }

    #[async_std::test]
    async fn test_map_operator() {
        let path =
            std::env::var("WORKFLOW_WASM").unwrap_or("../examples/map_op_mock.wasm".to_string());
        let server = post("127.0.0.1:7890").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "role":"Software Developer",
                }
        });
        let result = run_workflow(input, path);
        let res =
            serde_json::from_value::<HashMap<i32, i32>>(result.result.get("Ok").unwrap().clone());
        let expected = HashMap::from([
            (1, 10000000),
            (2, 10000000),
            (3, 10000000),
            (4, 10000000),
            (5, 10000000),
        ]);
        assert_eq!(res.unwrap(), expected)
    }

    #[async_std::test]
    async fn test_employee_salary_with_concat_operator() {
        let path = std::env::var("WORKFLOW_WASM")
            .unwrap_or("../examples/employee_salary_mock.wasm".to_string());
        let server = post("127.0.0.1:1234").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "role":"Software Developer",
                }
        });
        let result = run_workflow(input, path);
        assert!(result
            .result
            .to_string()
            .contains("Salary creditted for emp id 1 from Hugobyte"))
    }

    #[cfg(test)]
    mod flow_macro_tests {
        use dyn_clone::{clone_trait_object, DynClone};
        use serde_json::Value;
        use std::fmt::Debug;
        use workflow_macro::Flow;

        pub trait Execute: Debug + DynClone {
            fn execute(&mut self) -> Result<(), String>;
            fn get_task_output(&self) -> Value;
            fn set_output_to_task(&mut self, inp: Value);
        }

        clone_trait_object!(Execute);
        #[derive(Debug, Flow)]
        #[allow(dead_code)]
        pub struct WorkflowGraph {
            edges: Vec<(usize, usize)>,
            nodes: Vec<Box<dyn Execute>>,
        }

        impl WorkflowGraph {
            pub fn new(size: usize) -> Self {
                WorkflowGraph {
                    nodes: Vec::with_capacity(size),
                    edges: Vec::new(),
                }
            }
        }

        #[test]
        fn test_macro() {
            let workflow = WorkflowGraph::new(5);
            assert_eq!(0, workflow.node_count())
        }

        #[test]
        fn test_flow_macro_add_node() {
            #[derive(Debug, Clone, Default)]
            #[allow(dead_code)]
            pub struct Action {
                action_name: String,
                input: String,
                output: Value,
            }
            #[allow(unused_variables)]
            impl Execute for Action {
                fn execute(&mut self) -> Result<(), String> {
                    todo!()
                }
                fn get_task_output(&self) -> Value {
                    todo!()
                }
                fn set_output_to_task(&mut self, inp: Value) {
                    todo!()
                }
            }
            let node = Action::default();
            let mut workflow = WorkflowGraph::new(5);
            let _s = workflow.add_node(Box::new(node));
            assert_eq!(1, workflow.node_count())
        }
    }
}
