from hooks.common import out_put_method, convert_to_pascalcase, create_flow_objects, create_initialization_object, setter_no_op, new_method_gen
from hooks.functions import create_main_struct
import sys
import shutil
from pathlib import Path
import unittest
import os
from tackle import tackle
import warnings


sys.path.insert(0, os.path.dirname(__file__))


task_hook_data = [{'task_name': 'cartype', 'kind': 'OpenWhisk', 'input_args': [{'name': 'car_type', 'type': 'String'}], 'output_args': [{'name': 'car_company_list', 'type': 'HashMap<String,Vec<String>>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}, {'task_name': 'modelavail', 'kind': 'OpenWhisk', 'input_args': [{'name': 'car_company_list', 'type': 'HashMap<String,Vec<String>>'}, {'name': 'company_name', 'type': 'String'}], 'output_args': [{'name': 'models', 'type': 'Vec<String>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}},
                  {'task_name': 'modelsprice', 'kind': 'OpenWhisk', 'input_args': [{'name': 'models', 'type': 'Vec<String>'}], 'output_args': [{'name': 'model_price_list', 'type': 'HashMap<String,i32>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}, {'task_name': 'purchase', 'kind': 'OpenWhisk', 'input_args': [{'name': 'model_price_list', 'type': 'HashMap<String,i32>'}, {'name': 'model_name', 'type': 'String'}, {'name': 'price', 'type': 'i32'}], 'output_args': [{'name': 'message', 'type': 'String'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}]

flow_hook_data = actual_data = [{'type': 'Init', 'task_name': 'Cartype', 'depends_on': None}, {'type': 'Pipe', 'task_name': 'Modelavail', 'depends_on': {'operation': None, 'task': [{'name': 'cartype', 'fields': ['car_company_list']}]}}, {
    'type': 'Pipe', 'task_name': 'Modelsprice', 'depends_on': {'operation': None, 'task': [{'name': 'modelavail', 'fields': ['models']}]}}, {'type': 'Term', 'task_name': 'Purchase', 'depends_on': {'task': [{'name': 'modelsprice', 'fields': ['model_price_list']}]}}]


workflow_hook_data = {'workflow': {'name': 'workflow', 'version': '0.0.1'}, 'flows': [{'type': 'Init', 'task_name': 'Cartype', 'depends_on': None}, {'type': 'Pipe', 'task_name': 'Modelavail', 'depends_on': {'operation': None, 'task': [{'name': 'cartype', 'fields': ['car_company_list']}]}}, {'type': 'Pipe', 'task_name': 'Modelsprice', 'depends_on': {'operation': None, 'task': [{'name': 'modelavail', 'fields': ['models']}]}}, {'type': 'Term', 'task_name': 'Purchase', 'depends_on': {'task': [{'name': 'modelsprice', 'fields': ['model_price_list']}]}}], 'tasks': [{'task_name': 'cartype', 'kind': 'OpenWhisk', 'input_args': [{'name': 'car_type', 'type': 'String'}], 'output_args': [{'name': 'car_company_list', 'type': 'HashMap<String,Vec<String>>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}, {'task_name': 'modelavail', 'kind': 'OpenWhisk', 'input_args': [{'name': 'car_company_list', 'type': 'HashMap<String,Vec<String>>'}, {'name': 'company_name', 'type': 'String'}], 'output_args': [
    {'name': 'models', 'type': 'Vec<String>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}, {'task_name': 'modelsprice', 'kind': 'OpenWhisk', 'input_args': [{'name': 'models', 'type': 'Vec<String>'}], 'output_args': [{'name': 'model_price_list', 'type': 'HashMap<String,i32>'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}, {'task_name': 'purchase', 'kind': 'OpenWhisk', 'input_args': [{'name': 'model_price_list', 'type': 'HashMap<String,i32>'}, {'name': 'model_name', 'type': 'String'}, {'name': 'price', 'type': 'i32'}], 'output_args': [{'name': 'message', 'type': 'String'}], 'properties': {'api_host': 'https://65.20.70.146:31001', 'auth_token': '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP', 'insecure': 'true', 'namespace': 'guest'}}]}


def clean():
    out_path = Path.home()
    path = os.path.join(out_path, "output/")
    shutil.rmtree(path, ignore_errors=True)


class TestWorkFlow(unittest.TestCase):
    def test_case_conversion(self):

        test_string = "hello_world"

        actual_data = "HelloWorld"

        self.assertEqual(actual_data, convert_to_pascalcase(test_string))

    def test_task_hook_data(self):

        output = tackle('config.yaml')
        clean()

        self.assertEqual(output['workflows']['tasks'], task_hook_data)

    def test_flow_hook_data(self):

        output = tackle('config.yaml')
        clean()

        self.assertEqual(output['workflows']['flows'], flow_hook_data)

    def test_worflow_hook_data(self):

        output = tackle('config.yaml')

        clean()

        self.assertEqual(output['workflows'], workflow_hook_data)

    def test_code_generated(self):

        output = tackle('config.yaml')

        task_name = output['workflows']['tasks'][0]['task_name']

        result = out_put_method("", task_name)
        actual_data = f"""
fn output(&self) ->Value{{
    self.output.clone()
}}
"""

        self.assertEqual(result, actual_data)

        clean()

    def test_create_flow_objects(self):
        output = tackle('config.yaml')

        task = output['workflows']['tasks'][0]
        actual_data = f"""
let {task['task_name'].lower()}_index = workflow.add_node(Box::new({task['task_name'].lower()}));"""

        clean()

        self.assertEqual(actual_data, create_flow_objects(task))

    def test_struct_generated(self):
        output = tackle('config.yaml')

        task_name = output['workflows']['tasks'][0]['task_name']
        task_properties = output['workflows']['tasks'][0]['properties']
        task_kind = output['workflows']['tasks'][0]['kind']

        actual_data = create_main_struct(
            task_name, task_properties, "", task_kind)
        result = f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize,OpenWhisk)]
#[AuthKey="23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost="https://65.20.70.146:31001"]
#[Insecure="true"]
#[Namespace="guest"]

pub struct cartype{{
    action_name: String,
    pub input:cartypeInput,
    pub output:Value,
}}
"""
        clean()
        self.assertTrue(actual_data.casefold, result.casefold())

    def test_create_initialization_object(self):

        output = tackle('config.yaml')

        task_name = output['workflows']['tasks'][0]['task_name']

        actual_data_with_fields = f"""
let cartype= Cartype::new(String::from("cartype"));
        """

        actual_data_without_fields = f"""
let cartype = Cartype::new(some_field,String::from("cartype"));
    """

        result_with_fields = create_initialization_object(task_name, "")

        result_without_fields = create_initialization_object(
            task_name, "some_field,")

        clean()

        self.assertEqual(actual_data_with_fields.strip().casefold(),
                         result_with_fields.strip().casefold())

        self.assertEqual(actual_data_without_fields.strip().casefold(),
                         result_without_fields.strip().casefold())

    def test_setter_no_op(self):

        actual_data = f"""
fn setter(&mut self, value: Value) {{
        let value = value.get("field").unwrap();
        self.input.field = serde_json::from_value(value.clone()).unwrap();
}}
""".strip().casefold()

        result = setter_no_op("", "field").strip().casefold()

        self.assertEqual(actual_data, result)

    def test_new_method_generator(self):
        output = tackle('config.yaml')

        clean()

        task_name = output['workflows']['tasks'][0]['task_name']

        actual_data = f"""
pub fn new(action_name:String) -> Self {{ Self{{  input:cartypeInput{{..Default::default()}},action_name: action_name, ..Default::default()}}}}
        """.strip().casefold()

        result = new_method_gen(
            "", "", task_name).strip().casefold()

        self.assertEqual(actual_data, result)


if __name__ == "__main__":
    unittest.main()
