# Openwhisk Rust Macro

## Description

This project Openwhisk Rust Macro is a Rust derive macro which implements necessary methods to interact with [Openwhisk-rust](https://crates.io/crates/openwhisk-rust) library to invoke actions deployed in Openwhisk.
The Openwhisk Rust Client library requires [Rust](https://www.rust-lang.org/tools/install) to be installed on your local machine.

## Setup

 Add following libraries.
 ``` 
 openwhisk-rust  = "0.1.2"
 openwhisk_macro = "0.1.6" 
 ``` 
 In your `Cargo.toml` file of your rust package. 
 
 Then access those libraries by importing.
 
  ``` 
use openwhisk_macro::OpenWhisk;
use openwhisk-rust::*;
  ```   

## Usage

#### Creates necessary methods to invoke Openwhisk Actions.
 
 ``` 
#[derive(OpenWhisk)]
#[AuthKey = "Your auth key"]
#[ApiHost = "Host api endpoint url"]
#[Insecure = "true/false"]
#[Namespace = "Your namespace name"]

// Note: Action should be deployed prior,before accessing it. 
pub struct AnyAction {
action_name: String,
// *some fileds //
}

```
    

## References

* Learn more about  [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) .
* [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

#### License
Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)