#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(forgetting_copy_types)]
#![allow(unused_mut)]
#![allow(unused_must_use)]

mod common;
mod traits;
mod types;
use derive_enum_from_into::{EnumFrom, EnumTryInto};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use workflow_macro::Flow;

use common::*;
use paste::*;
use std::convert::TryInto;
use traits::*;
use types::*;
extern crate alloc;
use codec::{Decode, Encode};
use core::alloc::Layout;


#[no_mangle]
pub fn _start(ptr: *mut u8, length: i32) {
    let result: Value;
    unsafe {
        let mut vect = Vec::new();
        for i in 1..=length {
            if let Some(val_back) = ptr.as_ref() {
                vect.push(val_back.clone());
            }
            *ptr = *ptr.add(i as usize);
        }
        result = serde_json::from_slice(&vect).unwrap();
    }

    let res = main(result);
    let output = Output {
        result: serde_json::to_value(res).unwrap(),
    };
    let serialized = serde_json::to_vec(&output).unwrap();
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    std::mem::forget(ptr);
    unsafe {
        set_output(ptr as i32, size);
    }
}
