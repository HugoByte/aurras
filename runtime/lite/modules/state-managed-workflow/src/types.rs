use std::borrow::Borrow;

use super::*;
use openwhisk_macro::*;
use openwhisk_rust::*;

make_input_struct!(
EmployeeIdsInput,
[role:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
GetsalariesInput,
[id:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
SalaryInput,
[details:HashMap<i32,(i32,String)>],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
GetaddressInput,
[id:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);

make_main_struct!(
    EmployeeIds,
    EmployeeIdsInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",Namespace:"guest",ApiHost:"http://127.0.0.1:1234"],
    output
);
impl_new!(
    EmployeeIds,
    EmployeeIdsInput,
    [role:String]
);

make_main_struct!(
    Getsalaries,
    GetsalariesInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Namespace:"guest",ApiHost:"http://127.0.0.1:1234"],
    mapout
);
impl_new!(Getsalaries, GetsalariesInput, []);

make_main_struct!(
    Salary,
    SalaryInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [ApiHost:"http://127.0.0.1:1234",Namespace:"guest",Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"],
    output
);
impl_new!(Salary, SalaryInput, []);

make_main_struct!(
    Getaddress,
    GetaddressInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Namespace:"guest",ApiHost:"http://127.0.0.1:1234",Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"],
    mapout
);
impl_new!(Getaddress, GetaddressInput, []);
impl_setter!(EmployeeIds, []);
impl_map_setter!(Getsalaries, id:"ids", i32, "salary");
impl_concat_setter!(Salary, details);
impl_map_setter!(Getaddress, id:"ids", i32, "address");

make_input_struct!(
Input,
[role:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
impl_execute_trait!(EmployeeIds, Getsalaries, Salary, Getaddress);

#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT, "employee_salary_id");
    workflow.state_manger.update_workflow_initialized();

    let input: Input =
        serde_json::from_value(args.get("data").unwrap().clone()).map_err(|e| e.to_string())?;
    let prev_output: Vec<Value> = serde_json::from_value(args.get("prev_output").unwrap().clone())
        .map_err(|e| e.to_string())?;

    let employee_ids = EmployeeIds::new(input.role, "employee_ids".to_string());
    let getsalaries = Getsalaries::new("getsalaries".to_string());
    let salary = Salary::new("salary".to_string());
    let getaddress = Getaddress::new("getaddress".to_string());

    // basically [0, 1, 2, 3]. Because nodes are added based on the topological order
    let employee_ids_index = workflow.add_node(Box::new(employee_ids));
    let getsalaries_index = workflow.add_node(Box::new(getsalaries));
    let getaddress_index = workflow.add_node(Box::new(getaddress));
    let salary_index = workflow.add_node(Box::new(salary));

    workflow.add_edges(&[
        (employee_ids_index, getsalaries_index),
        (employee_ids_index, getaddress_index),
        (getsalaries_index, salary_index),
        (getaddress_index, salary_index),
    ]);

    for (i, val) in prev_output.iter().enumerate() {     
        let mut node = workflow.get_task_as_mut(i);
        node.set_result_output(val.clone());
        let action_name = node.get_action_name();
        workflow.state_manger.update_restore_sucess(&action_name, i as isize, val.clone())
    }

    for i in prev_output.len()..workflow.node_count() {
        workflow.run(i)?;
    }

    let len = workflow.node_count();
    let output = workflow.get_task(len - 1).get_task_output();
    let result = serde_json::to_value(output).unwrap();

    // std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(result)
}
