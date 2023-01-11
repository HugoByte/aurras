// Copyright 2023  HugoByte AI Labs Pvt Ltd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

/* Sample Generated structs form Yaml */

[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
// Properties of task

/*

Example:

#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://65.20.70.146:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]

*/


pub struct {{task_name}} {
    action_name: String,
    pub input: Task1Input,
    pub output: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct {{task_name}}Input {
    {{field_name}}: {{field_type}},
}

/*
Method Implementation for Task1
*/

impl Task1{
    pub fn new({{field_name}}:{{field_type}},action_name: String) -> Self{}

     fn setter(&mut self, value: Value) {
         let value = value.get("{field}").unwrap();
        self.input.{{field}} = serde_json::from_value(value.clone()).unwrap();
    }

    fn output(&self) -> Value {
        self.output.clone()
    }


}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Input {
   
    pub {{field_name}}: {{field_type}},
   
}