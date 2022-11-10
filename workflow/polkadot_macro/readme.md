# Substrate Macro

## Description

This is a Derive Macro for Workflow-Aurras. This macro implement functionalities for interaction between substrate based chain

## Usage

#### Creates necessary methods to for Workflow.
 
 ``` 
#[derive(Polkadot)]
#[Chain = "Westend"]
#[Operation = "transfer"]
pub struct Data {
    url: String,
    #[serde(default)]
    owner_key: String,
    #[serde(default)]
    op_values: Transactioninput,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Transactioninput {
    address: String,
    #[serde(default)]
    amount: u32,
    #[serde(default)]
    era: u32,
}

```
    

## References

* Learn more about  [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) .
* [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

#### License
Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)