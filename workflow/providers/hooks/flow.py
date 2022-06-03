import os
from psutil import getloadavg
from pydantic import Field
from tackle import BaseHook
from typing import Any, List
from .functions import convert_to_pascalcase

flow_list = []

class Flow(BaseHook):
    hook_type: str = 'flow'
    type: str = Field(..., description="Type of the flow")
    task_name: str = Field(..., description="Task Name")
    depends_on: Any = Field(..., description="Dependent Task")

    def exec(self):
        
        flow = {
            "type": self.type,
            "task_name": convert_to_pascalcase(self.task_name),
            "depends_on": self.depends_on,
        }

        flow_list.append(flow)

        

        return

def getFlows() -> list:
    global flow_list

    return flow_list