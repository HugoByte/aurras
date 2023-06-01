extern crate proc_macro;

use quote::*;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[cfg(not(tarpaulin_include))]
#[proc_macro_derive(Flow)]
pub fn derive_workflow(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_workflow(ast)
}

#[cfg(not(tarpaulin_include))]
fn impl_workflow(ast: DeriveInput) -> TokenStream {
    let workflow = ast.ident;

    let methods = quote! {


        impl #workflow {
            
            pub fn node_count(&self) -> usize {
                self.nodes.len()
            }

            pub fn add_node(&mut self, task: Box<dyn Execute>) -> usize {
                let len = self.nodes.len();
                self.nodes.push(task);
                len
            }

            pub fn add_edge(&mut self, parent: usize, child: usize) {
                self.edges.push((parent, child));
            }

            pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
                edges
                    .iter()
                    .for_each(|(source, destination)| self.add_edge(*source, *destination));
            }

            pub fn get_task(&self, index: usize) -> &Box<dyn Execute> {
                self.nodes.get(index).unwrap()
            }

            pub fn get_task_as_mut(&mut self, index: usize) -> &mut Box<dyn Execute> {
                self.nodes.get_mut(index).unwrap()
            }

            pub fn node_indices(&self) -> Vec<usize> {
                (0..self.node_count()).collect::<Vec<_>>()
            }

            pub fn init(&mut self) -> Result<&mut Self,String> {
                match self.get_task_as_mut(0).execute(){
                    Ok(()) => Ok(self),
                    Err(err) => Err(err)
                }

            }
            pub fn term(&mut self, task_index: Option<usize>) -> Result<Value,String> {

                match task_index {
                    Some(index) => {
                        let previous_index = (index - 1).try_into().unwrap();
                        let previous_task = self.get_task(previous_index);
                        let previous_task_output = previous_task.get_task_output();
                        let current_task = self.get_task_as_mut(index);
                        current_task.set_output_to_task(previous_task_output);
                        match current_task.execute(){
                            Ok(()) => Ok(current_task.get_task_output()),
                            Err(err) => Err(err),
                        }

                    },
                    None => {
                        let len = self.node_count();
                        Ok(self.get_task(len-1).get_task_output())
                    },
                }

            }

            pub fn pipe(&mut self, task_index: usize) -> Result<&mut Self,String> {
                let mut list = Vec::new();
                let edges_list = self.edges.clone();
                edges_list.iter().for_each(|(source, destination)| {
                    if destination == &task_index {
                        list.push(source)
                    }
                });
                let mut res: Vec<Value> = Vec::new();
                match list.len() {
                    0 => {
                        match self.get_task_as_mut(task_index).execute(){

                            Ok(()) => Ok(self),
                            Err(err) => Err(err),

                        }
                    },
                    1 => {
                        let previous_task_output = self.get_task(*list[0]).get_task_output();
                        let current_task = self.get_task_as_mut(task_index);
                        current_task.set_output_to_task(previous_task_output);
                        match current_task.execute(){
                        Ok(()) => Ok(self),
                        Err(err) => Err(err),
                        }
                    }
                    _ => {
                        res = list
                            .iter()
                            .map(|index| {
                                let previous_task = self.get_task(**index);
                                let previous_task_output = previous_task.get_task_output();
                                previous_task_output
                            })
                            .collect();

                        let s: Value = res.into();
                        let current_task = self.get_task_as_mut(task_index);
                        current_task.set_output_to_task(s);

                        match current_task.execute(){
                        Ok(()) => Ok(self),
                        Err(err) => Err(err),
                }
                    }
                }
            }
        }

    };
    methods.into()
}
