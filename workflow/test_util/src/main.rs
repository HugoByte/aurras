use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs,
    sync::{Arc, Mutex},
};
#[cfg(test)]
use std::collections::HashMap;
use wasi_common::WasiCtx;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
mod wasi_http;
use std::convert::TryInto;
use wasi_http::HttpCtx;
pub mod helper;
pub use helper::*;
use wasmtime::Linker;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MainInput {
    allowed_hosts: Option<Vec<String>>,
    data: Value,
}

fn main() {}

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
        .unwrap_or("../examples/CarMarketPlace_mock.wasm".to_string());
    let server = post().await;
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
    let path = std::env::var("WORKFLOW_WASM").unwrap_or("../examples/map_op_mock.wasm".to_string());
    let server = map().await;
    let input = serde_json::json!({
        "allowed_hosts": [
            server.uri()
        ],
        "data": {
            "role":"Software Developer",
            }
    });
    let result = run_workflow(input, path);
    let res = serde_json::from_value::<HashMap<i32, i32>>(result.result.get("Ok").unwrap().clone());
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
async fn test_employee_salray_with_concat_operator() {
    let path =
        std::env::var("WORKFLOW_WASM").unwrap_or("../examples/employe_salry_mock.wasm".to_string());
    let server = concat().await;
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
