use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod help;
pub use help::*;
mod tests;
mod types;

pub use types::*;
use state_manager::{GlobalState, GlobalStateManager, WorkflowState};

use cached::proc_macro::cached;
use cached::stores::RedisCache;
use cached::SizedCache;
use sha256::digest;
use std::{
    fs,
    sync::{Arc, Mutex},
};

use wasi_common::WasiCtx;
use wasi_experimental_http_wasmtime::{HttpCtx, HttpState};
use wasmtime::Linker;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

#[allow(dead_code)]
#[cached(
        type = "SizedCache<String, Output>",
        create = "{ SizedCache::with_size(10) }",
        convert = r###"{ hash_key.clone() }"###,
        result = true   // cache only if result is Ok
    )]
fn run_workflow_helper(
    data: Value,
    path: String,
    hash_key: String,
    state_manager: &mut GlobalState<WorkflowState>,
    workflow_index: usize,
) -> Result<Output, String> {
    use cached::IOCached;

    // need a disk storage to store internal state data
    let redis_cache: RedisCache<String, Value> = RedisCache::new("workflow".to_string(), 10 * 60)
        .set_connection_string("redis://127.0.0.1:6379")
        .set_refresh(true)
        // .set_namespace("workflows")
        .build()
        .unwrap();

    let prev_internal_state_data = match redis_cache.cache_get(&hash_key).unwrap() {
        Some(data) => data,
        None => serde_json::json!([]),
    };

    let wasm_file = fs::read(path).unwrap();
    let mut input: MainInput = serde_json::from_value(data).unwrap();

    input.data = serde_json::json!({"data": input.data, "prev_output": prev_internal_state_data});

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
        .func_wrap("host", "get_prev_output", move || -> i32 { mem_size })
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
                    Ok(()) => match serde_json::from_slice::<Output>(&buffer) {
                        Ok(serialized_output) => {
                            let mut output = output.lock().unwrap();
                            *output = serialized_output;
                            Ok(())
                        }
                        Err(err) => {
                            let msg = format!("failed to serialize host memory: {}", err);
                            Err(Trap::new(msg))
                        }
                    },
                    _ => Err(Trap::new("failed to read host memory")),
                }
            },
        )
        .expect("should define the function");

    let output_2: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let output_ = output_2.clone();

    linker
        .func_wrap(
            "host",
            "set_state",
            move |mut caller: Caller<'_, WasiCtx>, ptr: i32, capacity: i32| {
                let output_2 = output_.clone();
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return Err(Trap::new("failed to find host memory")),
                };
                let offset = ptr as u32 as usize;
                let mut buffer: Vec<u8> = vec![0; capacity as usize];
                match mem.read(&caller, offset, &mut buffer) {
                    Ok(()) => match serde_json::from_slice::<Value>(&buffer) {
                        Ok(task_state_data) => {
                            // execution_state =
                            let execution_state: String = serde_json::from_value(
                                task_state_data.get("execution_state").unwrap().clone(),
                            )
                            .unwrap();

                            // storing if it is success
                            if &execution_state == "Success" {
                                let mut output_2 = output_2.lock().unwrap();
                                let output_data = task_state_data.get("output").unwrap().clone();

                                if !output_data.is_null() {
                                    output_2.push(output_data);
                                }
                            }

                            Ok(())
                        }
                        Err(err) => {
                            let msg = format!("failed to serialize host memory: {}", err);
                            Err(Trap::new(msg))
                        }
                    },
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

    let allowed_hosts = input.allowed_hosts;
    let http_ctx = HttpCtx {
        allowed_hosts,
        max_concurrent_requests,
    };
    let http_state = HttpState::new().unwrap();

    http_state
        .add_to_linker(&mut linker, move |_store| http_ctx.clone())
        .unwrap();

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

    state_manager.update_running(workflow_index).unwrap();
    let _result_from_wasm = run.call(&mut store, (data_ptr, len));

    let malloc = linking
        .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "free_memory")
        .unwrap();
    malloc
        .call(&mut store, (data_ptr, data.len() as i32, 2))
        .unwrap();

    let res = output.lock().unwrap().clone();

    let state_output = output_2.lock().unwrap().clone();

    let state_data_val = serde_json::to_value(state_output).unwrap();
    redis_cache.cache_set(hash_key, state_data_val).unwrap();

    if res.result.get("Err").is_some() {
        state_manager
            .update_result(workflow_index, res.result.clone(), false)
            .unwrap();
    } else {
        state_manager
            .update_result(workflow_index, res.result.clone(), true)
            .unwrap();
    }

    Ok(res)
}

pub fn run_workflow(data: Value, path: String) -> Result<Output, String> {
    let mut state_manager = GlobalState::new();

    state_manager.new_workflow(0, &path);

    let digest = digest(format!("{:?}{:?}", data, path));
    run_workflow_helper(data, path, digest, &mut state_manager, 0)
}
