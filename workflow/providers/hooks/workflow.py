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
    action_properties: Any = Field(..., title="action_properties",
                                   description="Porperties for the actions defined")

    def exec(self):
        task_list = getTasks()
        flow_list = getFlows()

        global task_store
        global task_store_copy
        global action_properties

        workflow_config = create_workflow_config(self.name,self.version)

        action_properties = self.action_properties
        generate_dependency_matrix(flow_list)
        struct_generator(task_list, action_properties)

        
        create_main_function(task_list,action_properties)
        generate_output(workflow_config)

        return
