import os
from pydantic import Field
from tackle import BaseHook, tackle
from typing import Any, List, Dict
from .functions import convert_to_pascalcase

task_list = []


class Task(BaseHook):

    hook_type: str = 'task'
    kind: str = Field(..., title='kind', description="To Specify kind of task")
    name: str = Field(..., title='name',
                      description="Name of the action deployed to OpenWhisk")
    input_args: List = Field(..., title='input_args',
                             description="List containing dictionary of parameter and type and input")
    output_args: List = Field(..., title='output_args',
                              description="List containing dictionary of parameter and type for output")
    properties: Dict = Field(..., title="property",
                             description="Dictionary of property")

    def exec(self):

        task = {
            "task_name": self.name,
            "kind": self.kind,
            "input_args": self.input_args,
            "output_args": self.output_args,
            "properties": self.properties
        }

        task_list.append(task)

        return


def getTasks() -> list:
    global task_list
    return task_list
