// Copyright 2023 HugoByte AI Labs Pvt Ltd
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

/*
All the task generate will implement this methods

*/

use super::*;

pub trait Execute: Debug + DynClone {
    fn execute(&mut self) -> Result<(), String>;
    fn get_task_output(&self) -> Types;
    fn set_output_to_task(&mut self, inp: Types);
}

clone_trait_object!(Execute);
