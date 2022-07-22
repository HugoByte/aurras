import os
from pydantic import Field
from tackle import BaseHook, tackle
from typing import Any, List
from .functions import convert_to_pascalcase

task_list = []

class Task(BaseHook):
   
    hook_type: str = 'task'
    kind: str = Field(..., title='kind', description="To Specify kind of task")
    name: str = Field(..., title='name',
                      description="Name of the action deployed to openwhisk")
    input_args: List = Field(..., title='input_args',
                             description="List containing dictonary of parameter and type and input")
    output_args: List = Field(..., title='output_args',
                              description="List contating dictonary of paramter and type for output")
    def exec(self):
        
        task = {
            "task_name": convert_to_pascalcase(self.name),
            "kind": self.kind,
            "input_args": self.input_args,
            "output_args": self.output_args
        }

        task_list.append(task)
        
        
        return 

def getTasks() -> list:
    global task_list

    return task_list
