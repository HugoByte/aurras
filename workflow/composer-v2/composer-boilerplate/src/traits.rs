
use super::*;

pub trait Execute : Debug + DynClone  {
    fn execute(&mut self)-> Result<(),String>;
    fn get_task_output(&self)->Value;
    fn set_output_to_task(&mut self, inp: Value);
}

clone_trait_object!(Execute);
