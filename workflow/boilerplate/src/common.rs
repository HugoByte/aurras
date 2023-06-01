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

// WorkflowGraph Struct
use super::*;
#[derive(Debug, Flow)]
pub struct WorkflowGraph {
    edges: Vec<(usize, usize)>,
    nodes: Vec<Box<dyn Execute>>,
}

//Methods Impl for WorkflowGraph
impl WorkflowGraph {
    pub fn new(size: usize) -> Self {
        WorkflowGraph {
            nodes: Vec::with_capacity(size),
            edges: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! impl_execute_trait {
    ($ ($struct : ty), *) => {

            paste!{
                $( impl Execute for $struct {
                    fn execute(&mut self) -> Result<(),String>{
        self.run()
    }

    fn get_task_output(&self) -> Types {
        self.output().clone().into()
    }

    fn set_output_to_task(&mut self, input: Types) {
        self.setter(input)
    }
                }
            )*
        }
    };
}

#[allow(dead_code, unused)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &second {
            if key.clone() == s_key.to_owned() {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}

#[test]
fn test_insert(){
    let data = HashMap::from([(1, "test")]);
    let data1 = HashMap::from([(1, "test1")]);
    let res = join_hashmap(data, data1);
    assert_eq!(HashMap::from([(1, ("test", "test1"))]), res);
}