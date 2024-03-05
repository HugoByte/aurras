// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use std::collections::HashMap;
// use std::convert::TryInto;
// use std::fs;
// use std::{
//     sync::{Arc, Mutex},
// };
// use wasi_common::WasiCtx;
// use wasmtime::{Caller, Config, Engine, Linker, Module, Store, Trap, Extern};
// use wasmtime_wasi::sync::WasiCtxBuilder;
// use wasmtime_wasi_http::WasiHttpCtx;
// use wasi_http::*;
// use wasi_experimental_http::*;


// pub mod help;
// pub use help::*;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MainInput {
//     allowed_hosts: Option<Vec<String>>,
//     data: Value,
// }

// #[allow(dead_code)]
// pub fn run_wasm(wasm_path: &str, function_name: &str, data: Value,) -> wasmtime::Result<()> {
//     // Load the Wasm module from a file
//     let wasm_bytes = fs::read(wasm_path)?;

// // let input = MainInput { function_name: function_name.to_string() };
// let input: MainInput = serde_json::from_value(data).unwrap();
//     // Create a Wasmtime engine with WASI support
//     let engine = Engine::default();

//     // Compile the Wasm module
//     let module= Module::from_binary(&engine, &wasm_bytes)?;
//     let mut linker = Linker::new(&engine);

//     let output: Arc<Mutex<Output>> = Arc::new(Mutex::new(Output {
//         result: serde_json::json!({}),
//     }));
//     let output_ = output.clone();
//     let buf = serde_json::to_vec(&input).expect("should serialize");
//     let mem_size: i32 = buf.len() as i32;

//     // fn get_input_size(mem_size: i32) -> i32 {
//     //     mem_size
//     // }
    

//     linker
//         .func_wrap("host", "get_input_size", move || -> i32 { mem_size })
//         .expect("should define the function");

//         linker
//         .func_wrap(
//             "host",
//             "set_output",
//             move |mut caller: Caller<'_, WasiCtx>, ptr: i32, capacity: i32| {
//                 let output = output_.clone();
//                 let mem = match caller.get_export("memory") {
//                     Some(Extern::Memory(mem)) => mem,
//                     _ => return Err(Trap::new("failed to find host memory")),
//                 };
//                 let offset = ptr as u32 as usize;
//                 let mut buffer: Vec<u8> = vec![0; capacity as usize];
//                 match mem.read(&caller, offset, &mut buffer) {
//                     Ok(()) => {
//                         println!(
//                             "Buffer = {:?}, ptr = {}, capacity = {}",
//                             buffer, ptr, capacity
//                         );
//                         match serde_json::from_slice::<Output>(&buffer) {
//                             Ok(serialized_output) => {
//                                 let mut output = output.lock().unwrap();
//                                 *output = serialized_output;
//                                 Ok(())
//                             }
//                             Err(err) => {
//                                 let msg = format!("failed to serialize host memory: {}", err);
//                                 Err(Trap::new(msg))
//                             }
//                         }
//                     }
//                     _ => Err(Trap::new("failed to read host memory")),
//                 }
//             },
//         )
//         .expect("should define the function");

//     wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;

//     // linker.func_wrap(
//     //     "host",
//     //     "main",
//     //     |caller: Caller<'_, WasiCtx>, args: Value| -> Result<(), wasmtime::Trap> {
//     //         println!("Got {:?} from WebAssembly", args);
//     //     },
//     // )?;

//     let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
//     // Create a new Wasmtime store
//     let mut store = Store::new(&engine, wasi_ctx);

//     let max_concurrent_requests = Some(42);

//     let http = HttpCtx::new(input.allowed_hosts, max_concurrent_requests).unwrap();
//     http.add_to_linker(&mut linker).unwrap();
   

//     // Instantiate the Wasm module
//     let instance = linker.instantiate(&mut store, &module)?;

//     // Execute the specified exported function
//     let result = instance.get_typed_func::<(), ()>(&mut store, function_name)?;

//     // Convert the result to a Vec<u8> for flexibility
//     let result_vec = result.call(&mut store, ())?;

//     Ok(result_vec)
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Output {
//     pub result: Value,
// }



// #[cfg(test)]
// mod tests {
//     use wasmtime_wasi::preview2::command::exports::wasi::cli::run;

//     use super::*;
//     use std::{
//         fs,
//         sync::{Arc, Mutex},
//     };

//     #[test]
//     fn test_run_wasm_module() {
//         let wasm_path =
//             "/Users/prathiksha/Downloads/Hugobyte/composer/test/output/car_market_place_0.0.1.wasm";
//         let function_name = "main";
//         // assert_eq!(run_wasm(wasm_path, function_name).is_ok(), true);
//     }

//     #[async_std::test]
//     async fn test_car_market() {
//         // let wasm_path =
//         //     "/Users/prathiksha/Downloads/Hugobyte/composer/test/output/car_market_place_0.0.1.wasm";
//             let path = std::env::var("WORKFLOW_WASM")
//             .unwrap_or("/Users/prathiksha/Downloads/Hugobyte/composer/test/output/car_market_place_0.0.1.wasm".to_string());
//             let server = post("127.0.0.1:8080").await;

//         let input = serde_json::json!({
//             "allowed_hosts": [
//                 server.uri()
//             ],
//             "data": {
//                 "car_type":"hatchback",
//                 "company_name":"maruthi",
//                 "model_name":"alto",
//                 "price":1200000
//                 }
//         });
//         let function_name = "main";
//         let result = run_wasm(&path.to_string(), &function_name.to_string(), input).is_ok();
//         print!("{:?}", result);

//         assert_eq!(result, true);
//     }
// }

use serde::{Deserialize, Serialize};
use serde_json::Value;
// #[cfg(test)]
// mod wasi_http;

pub mod help;
pub use help::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    allowed_hosts: Option<Vec<String>>,
    data: Value,
}

fn main() {}

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
    use wasmtime::Linker;
    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;
    use wasi_experimental_http::*;
    use http;
    use bytes::Bytes;

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

        let url = input.allowed_hosts;
        let req = http::request::Builder::new()
        .method(http::Method::POST)
        .uri(&url.unwrap_or_default().join("/").to_string())
        .header("Content-Type", "text/plain");
        // .header("abc", "def");
    
        let b = Bytes::from("Testing with a request body. Does this actually work?");
        let req = req.body(Some(b)).unwrap();

        let mut res = wasi_experimental_http::request(req).expect("cannot make request");

        let bytes = res.body_read_all().unwrap_or_else(|err| {
            eprintln!("Error reading response body: {:?}", err);
            Vec::new()
        });
        let str = std::str::from_utf8(&bytes).unwrap().to_string();
        // str.add_to_linker(&mut linker).unwrap();
        // let str = std::str::from_utf8(&res.body_read_all()).unwrap().to_string();

        // let http = HttpCtx::new(input.allowed_hosts, max_concurrent_requests).unwrap();
        // http.add_to_linker(&mut linker).unwrap();

        // let h = wasi_experimental_http::


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
            .unwrap_or("/Users/prathiksha/Downloads/Hugobyte/composer/test/output/car_market_place_0.0.1.wasm".to_string());
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
}