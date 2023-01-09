# Providers

## Workflow Config

### Structure of the Workflow Config

Workflow Config is a yaml file which is used to generate the rust code for workflow execution from `Tackle-Box`

Workflow Config has **3** different Sections

1.  `Tasks`

2.  `Flows`

3.  `Workflows`
  
### 1. Tasks

Tasks section of yaml contains the list of tasks which refers to actions that needs to be executed.

A task should have **4** fields

-  `Kind` is the type of the action which the task refers to.

-  `Name` denotes the name of your task.

-  `Input Arguments` you can pass the necessary input fields for the action to the input struct.

-  `Output Arguments` you can pass the necessary input fields for the action to the output struct.

-  `Properties` you can pass the properties that are related to the tasks.

### 2. Flows  

Flows section of Workflow Config contains list of flows which represents order of execution of the tasks

A flow should have **3** fields

1.  `Type` : To mention what will be order of execution of its task.

There are **3** types in flows  

-  `Init` : A Task that is executed at the beginning of the workflow. So for an init task, depends on must be null.

-  `Pipe` : A task which is dependent on previously executed actions.

-  `Term` : A task which will be the last task to be executed.

  
2.  `Task Name` : Name of the task in the flow.


3.  `Depends on` : This is where the actions required for the execution of the flow is mentioned.

  
### 3. Workflows

Workflow section of Workflow Config is to define the workflow.

Workflow should have 3 fields

-  `Workflow Name` : The name of the workflow.

-  `Version` : Version of the workflow


A Workflow Config file should be configured in the following order

1.  `Tasks` referring to actions.

2.  `Flows` containing order of execution of tasks.

3.  `Workflows` to define the workflow.

### Example

An example for a **Workflow Config** file can be found [here](workflow/examples)

## Operators  

There are **two types** of operators that can be used in a **flow**.

### `Map`  

**Map operator** is used to iterate through a given list on inputs for an action. The operator stores the outputs as a [hashmap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html), containing key-value pairs of the action input and its output.

### `Concat`

**Concat Operator** is used to combine the output of two actions which can be used as an input for other actions.  

## HOOKS

Hooks are used to generate rust code from Workflow Config.

We have **3** main hooks in the workflow

-  [Task Hook](https://github.com/HugoByte/aurras/blob/next/workflow/providers/hooks/task.py) to generate the Action Struct, Action Input Struct and Action Output Struct with its fields.

-  [Flow Hook](https://github.com/HugoByte/aurras/blob/next/workflow/providers/hooks/flow.py) to orchestrate the flow of the actions to be executed.

-  [Workflow Hook](https://github.com/HugoByte/aurras/blob/next/workflow/providers/hooks/workflow.py) for the creation of rust code with all required packages.

## Boilerplate

Boilerplate contains all traits, implementation of the flow structs, workflow struct and global imports which can be found [here](https://github.com/HugoByte/aurras/blob/next/workflow/providers/hooks/functions.py) .

### Prerequisites
  
[Tackle-box](https://pypi.org/project/tackle-box/)

## Usage

1. Clone the repository
`
git clone https://github.com/HugoByte/aurras.git
`

2. Change present working directory  to providers
`cd 
    workflow/providers`

3. Run the command 

	`tackle yourworkflowconfig.yaml`
  
4. Generated rust package for the workflow can be found in home directory under the name `output`.
  
## References

[Tackle-box](https://github.com/robcxyz/tackle-box/tree/tk-provider)

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)