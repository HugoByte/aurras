use serde::{Deserialize, Serialize};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
mod wasi_http;
use serde_json::{Error, Value};
use std::convert::TryInto;
use wasi_http::HttpCtx;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    allowed_hosts: Option<Vec<String>>,
    data: Value,
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input = serde_json::from_value::<Input>(args)?;

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);
    let module = Module::from_file(&engine, "workflow.wasm").unwrap();
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
        .get_typed_func::<(i32, i32), i32, _>(&mut store, "_start")
        .unwrap();
    let res = run.call(&mut store, (data_ptr, len)).unwrap();

    let mut res = res as usize;

    let mut buffer: [u8; 8] = [0; 8];

    let mut length_vec = Vec::new();

    for i in 0..8 {
        let out = memory.data(&mut store)[i as usize + res];
        length_vec.push(out);
    }

    for i in 0..length_vec.len() {
        buffer[i] = length_vec[i];
    }

    let lenght = u64::from_be_bytes(buffer);

    let mut data_vec = Vec::new();

    for i in 8..lenght + 8 {
        let out = memory.data(&mut store)[i as usize + res];
        data_vec.push(out);
    }

    let result: Value = serde_json::from_slice(&data_vec).unwrap();

    let malloc = linking
        .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "free_memory")
        .unwrap();
    malloc
        .call(&mut store, (data_ptr, data.len() as i32, 2))
        .unwrap();

    Ok(result)
}

#[test]
fn test_works() {
    let input = serde_json::json!({
        "allowed_hosts" : ["https://65.20.70.146:31001"],
        "data" : {
            "car_type":"hatchback",

            "company_name":"maruthi",

            "model_name":"alto",

            "price":1200000
        }
    });

    let res = main(input);

    println!("{:?}", res);
}
