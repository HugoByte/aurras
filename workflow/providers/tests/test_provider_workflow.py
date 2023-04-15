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

def clean():
    out_path = Path.home()
    path = os.path.join(out_path, "output/")
    shutil.rmtree(path, ignore_errors=True)


class TestWorkFlow(unittest.TestCase):
    def test_case_conversion(self):
        test_string = "hello_world"
        actual_data = "HelloWorld"
        self.assertEqual(actual_data, convert_to_pascalcase(test_string))

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

    def test_create_initialization_object(self):
        output = tackle('config.yaml')
        task_name = output['workflows']['tasks'][0]['task_name']
        task_name_pascal = convert_to_pascalcase(task_name)
        actual_data_with_fields = f"""
let {task_name_pascal}= {task_name_pascal}::new(String::from("{task_name}"));
        """
        actual_data_without_fields = f"""
let {task_name_pascal} = {task_name_pascal}::new(some_field,String::from("{task_name}"));
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
pub fn new(action_name:String) -> Self {{ Self{{  input:{task_name}input{{..Default::default()}},action_name: action_name, ..Default::default()}}}}
        """.strip().casefold()
        result = new_method_gen(
            "", "", task_name).strip().casefold()

        self.assertEqual(actual_data, result)
    
    def test_struct_generated(self):
        output = tackle('config.yaml')
        task_name = output['workflows']['tasks'][0]['task_name']
        task_properties = output['workflows']['tasks'][0]['properties']
        task_kind = output['workflows']['tasks'][0]['kind']
        actual_data = create_main_struct(
            task_name, task_properties, "", task_kind)
        result = f"""
pub struct {task_name}{{
    action_name: String,
    pub input:{task_name}Input,
    pub output:Value,
}}
"""
        clean()
        self.assertIn(result.casefold().replace(' ', ''),actual_data.casefold().replace(' ', ''))

    def test_struct_header(self):
        output = tackle('config.yaml')
        task_name = output['workflows']['tasks'][0]['task_name']
        task_properties = output['workflows']['tasks'][0]['properties']
        task_kind = output['workflows']['tasks'][0]['kind']
        actual_data = create_main_struct(
            task_name, task_properties, "", task_kind)
        result_header = f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
"""
        clean()
        if output['workflows']['tasks'][0]['kind'] == "OpenWhisk":
            result_header = f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize,OpenWhisk)]
"""
        elif output['workflows']['tasks'][0]['kind'] == "Polkadot":
            result_header = f"""
#[derive(Default, Debug, Clone, Serialize, Deserialize,Polkadot)]
""" 
        self.assertIn(result_header.casefold().replace(' ', ''),actual_data.casefold().replace(' ', ''))

if __name__ == "__main__":
    unittest.main()
