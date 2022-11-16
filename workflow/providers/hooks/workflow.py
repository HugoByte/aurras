from pydantic import Field
from tackle import BaseHook
from typing import Any
from .functions import struct_generator, create_main_function, generate_dependency_matrix, generate_output,create_workflow_config
from .task import getTasks
from .flow import getFlows


class WorkFlow(BaseHook):
    hook_type: str = 'workflow'
    name: str = Field(..., title='name', description='Workflow name')
    version: str = Field(..., title='version', description='Wrokflow Version')

    def exec(self):
        task_list = getTasks()
        flow_list = getFlows()
        
        workflow_config = create_workflow_config(self.name,self.version)

        generate_dependency_matrix(flow_list)
        struct_generator(task_list)

        create_main_function(task_list)
        generate_output(workflow_config, task_list)

        return
