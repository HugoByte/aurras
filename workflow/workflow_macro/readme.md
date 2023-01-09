# Workflow Macro

## Description

This is a Derive Macro for Workflow-Aurras. This macro implements necessary methods for workflow, like add node, get task, add edges, etc..

## Setup

 Add following libraries.
 ``` 
 workflow_macro = "0.0.2"
 ``` 
 In your `Cargo.toml` file of your rust package. 


 Access this Macro by importing.
 
  ``` 
use workflow_macro::Flow;
  ```   

## Usage

#### Creates necessary methods to for Workflow.
 
 ``` 
 #[derive(Flow)]
  pub struct Worflow{

    edges: Vec<(usize, usize)>,
    nodes: Vec<`node_type`>,

  }

```
    

## References

* Learn more about  [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) .
* [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

#### License
Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)