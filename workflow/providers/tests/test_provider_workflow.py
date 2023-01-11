from hooks.common import out_put_method
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

    def test_task_hook_data(self):

        output = tackle('config.yaml')

        self.assertEqual(output['workflows']['tasks'], task_hook_data)

        clean()

    def test_flow_hook_data(self):

        output = tackle('config.yaml')

        self.assertEqual(output['workflows']['flows'], flow_hook_data)

        clean()

    def test_worflow_hook_data(self):

        output = tackle('config.yaml')

        self.assertEqual(output['workflows'], workflow_hook_data)

        clean()

    def test_code_generated(self):

        output = tackle('config.yaml')

        task_name = output['workflows']['tasks'][0]['task_name']

        result = out_put_method("", task_name)
        actual_data = f"""
fn output(&self) ->cartypeOutput{{
    self.output.clone()
}}
"""

        self.assertEqual(result, actual_data)

        clean()


if __name__ == "__main__":
    unittest.main()
