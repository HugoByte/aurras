use super::{Parser, Result, SourceFiles};
use crate::errors;
use aurras_primitives::Exception;
use starlark::environment::FrozenModule;
use starlark::environment::{Globals, GlobalsBuilder, Module};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;
use std::path::{Path, PathBuf};

pub static FILE_EXTENSION: &str = "echo";
pub static ENTRY_FILE: &str = "main";

#[starlark_module]
fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn workflow(name: String) -> anyhow::Result<String> {
        Ok(name)
    }
}

#[derive(Default)]
pub struct Echo;

impl Echo {
    fn get_module(module: &str, files: &SourceFiles) -> Result<FrozenModule> {
        let ast: AstModule = AstModule::parse_file(
            files
                .files()
                .get(&PathBuf::from(module.replace(
                    "@base:/",
                    &files.base().to_str().unwrap()
                )))
                .unwrap(),
            &Dialect::Extended,
        )
        .unwrap();

        let mut loads = Vec::new();

        for (_, load) in ast.loads() {
            loads.push((load.to_owned(), Self::get_module(load, files)?));
        }

        let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();
        let mut loader = ReturnFileLoader { modules: &modules };

        let globals = GlobalsBuilder::new().with(starlark_workflow).build();
        let module = Module::new();

        let mut eval = Evaluator::new(&module);
        eval.set_loader(&mut loader);
        eval.eval_module(ast, &globals).unwrap();

        Ok(module.freeze().unwrap())
    }
}

impl Parser for Echo {
    fn parse(&self, files: &SourceFiles) {
        let main = Self::get_module(
            &format!(
                "{}/{}.{}",
                files.base().display(),
                ENTRY_FILE,
                FILE_EXTENSION
            ),
            files,
        )
        .unwrap();

        println!("{:?}", main.get("ab"));
    }
}
