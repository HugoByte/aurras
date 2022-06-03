from ntpath import join
import os
from pydantic import Field
from tackle import BaseHook
from typing import Any, List
from .functions import struct_generator, create_main_function, create_main_input_struct, generate_output
from .task import getTasks
from .flow import getFlows


class WorkFlow(BaseHook):
    hook_type: str = 'workflow'
    name: str = Field(..., title='name', description='Workflow name')
    version: str = Field(..., title='version', description='Wrokflow Version')
    action_properties: Any = Field(..., title="action_properties",
                                   description="Porperties for the actions defined")

    def exec(self):
        task_list = getTasks()
        flow_list = getFlows()

        global task_store
        global task_store_copy
        global action_properties

        action_properties = self.action_properties
        struct_generator(task_list, action_properties)

        create_main_input_struct(task_list, flow_list)
        create_main_function(task_list)
        generate_output()

        return
