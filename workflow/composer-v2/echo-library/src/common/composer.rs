use std::fs::OpenOptions;
use std::io::Write;

use anyhow::Ok;
use composer_primitives::types::SourceFiles;
use starlark::environment::FrozenModule;
use starlark::eval::ReturnFileLoader;

use super::*;

const COMMON: &str = include_str!("../../../composer-boilerplate/src/common.rs");
const LIB: &str = include_str!("../../../composer-boilerplate/src/lib.rs");
const TRAIT: &str = include_str!("../../../composer-boilerplate/src/traits.rs");
const CARGO: &str = include_str!("../../../composer-boilerplate/Cargo.toml");

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    pub config_files: Vec<String>,
    pub workflows: RefCell<Vec<Workflow>>,
    pub custom_types: RefCell<HashMap<String, String>>,
}

impl Composer {
    /// Adds config file to the composer
    /// This method is called by the user
    ///
    /// # Arguments
    ///
    /// * `config` - A string slice that holds the of the config file along with its name
    ///
    /// # Example
    ///
    /// ```
    /// use echo_library::Composer;
    /// let mut composer = Composer::default();
    /// composer.add_config("config/path/config_file_name_here");
    /// ```
    pub fn add_config(&mut self, config: &str) {
        self.config_files.push(config.to_string());
    }

    /// Adds a new workflow to the composer.
    /// This method is invoked by the workflows function inside the starlark_module.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the workflow to be added
    /// * `version` - Version of the workflow
    /// * `tasks` - HashMap of tasks associated with the workflow
    /// * `custom_types` - Optional vector of custom types names that are created within config
    ///   for the workflow.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Result indicating success if the workflow is added successfully,
    ///   or an error if the workflow name is empty or if there is a duplicate workflow name.
    ///
    pub fn add_workflow(
        &self,
        name: String,
        version: String,
        tasks: HashMap<String, Task>,
    ) -> Result<(), Error> {
        for workflow in self.workflows.borrow().iter() {
            if workflow.name == name {
                return Err(Error::msg("Workflows should not have same name"));
            }
        }
        if name.is_empty() {
            Err(Error::msg("Workflow name should not be empty"))
        } else {
            self.workflows.borrow_mut().push(Workflow {
                name,
                version,
                tasks,
            });
            Ok(())
        }
    }

    pub fn build(&self, verbose: bool, temp_dir: &PathBuf) -> Result<(), Error> {
        if verbose {
            Command::new("rustup")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["target", "add", "wasm32-wasi"])
                .status()?;

            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi"])
                .status()?;
        } else {
            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi", "--quiet"])
                .status()?;
        }
        Ok(())
    }

    fn copy_boilerplate(
        &self,
        temp_dir: &PathBuf,
        types_rs: &str,
        workflow_name: String,
        workflow: &Workflow,
    ) -> Result<PathBuf, Error> {
        let temp_dir = temp_dir.join(&workflow_name);
        let curr = temp_dir.join("boilerplate");

        std::fs::create_dir_all(curr.clone().join("src"))?;

        let src_curr = temp_dir.join("boilerplate/src");
        let temp_path = src_curr.as_path().join("common.rs");

        std::fs::write(temp_path, &COMMON[..])?;

        let temp_path = src_curr.as_path().join("lib.rs");
        std::fs::write(temp_path.clone(), &LIB[..])?;

        let mut lib = OpenOptions::new()
            .write(true)
            .append(true)
            .open(temp_path)?;

        let library = get_struct_stake_ledger(workflow);
        writeln!(lib, "{library}").expect("could not able to add struct to lib");

        let temp_path = src_curr.as_path().join("types.rs");
        std::fs::write(temp_path, types_rs)?;

        let temp_path = src_curr.as_path().join("traits.rs");
        std::fs::write(temp_path, &TRAIT[..])?;

        let cargo_path = curr.join("Cargo.toml");
        std::fs::write(cargo_path.clone(), &CARGO[..])?;

        let mut cargo_toml = OpenOptions::new()
            .write(true)
            .append(true)
            .open(cargo_path)?;

        let dependencies = generate_cargo_toml_dependencies(workflow);
        writeln!(cargo_toml, "{dependencies}")
            .expect("could not able to add dependencies to the Cargo.toml");

        Ok(temp_dir)
    }
}

impl Composer {
    pub fn compile(&self, module: &str, files: &SourceFiles) -> Result<FrozenModule, Error> {
        let ast: AstModule = AstModule::parse_file(
            files
                .files()
                .get(&PathBuf::from(format!(
                    "{}/{}",
                    files.base().display(),
                    module
                )))
                .ok_or_else(|| {
                    Error::msg(format!(
                        "FileNotFound at {}/{}",
                        files.base().display(),
                        module
                    ))
                })?,
            &Dialect::Extended,
        )?;

        let mut loads = Vec::new();

        for load in ast.loads() {
            loads.push((
                load.module_id.to_owned(),
                Self::compile(&self, load.module_id, files)?,
            ));
        }

        let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();
        let mut loader = ReturnFileLoader { modules: &modules };

        // We build our globals by adding some functions we wrote
        let globals = GlobalsBuilder::extended_by(&[
            StructType,
            RecordType,
            EnumType,
            Map,
            Filter,
            Partial,
            ExperimentalRegex,
            Debug,
            Print,
            Pprint,
            Breakpoint,
            Json,
            Typing,
            Internal,
            CallStack,
        ])
        .with(starlark_workflow_module)
        .with(starlark_datatype_module)
        .with_struct("Operation", starlark_operation_module)
        .build();

        let module = Module::new();

        let int = module.heap().alloc(RustType::Int);
        module.set("Int", int);
        let uint = module.heap().alloc(RustType::Uint);
        module.set("Uint", uint);
        let int = module.heap().alloc(RustType::Float);
        module.set("Float", int);
        let int = module.heap().alloc(RustType::String);
        module.set("String", int);
        let int = module.heap().alloc(RustType::Boolean);
        module.set("Bool", int);

        {
            let mut eval = Evaluator::new(&module);
            // We add a reference to our store
            eval.set_loader(&mut loader);
            eval.extra = Some(self);
            eval.eval_module(ast, &globals)?;
        }

        Ok(module.freeze()?)
    }

    pub fn build_directory(
        &self,
        build_path: &PathBuf,
        out_path: &PathBuf,
        quiet: bool,
    ) -> Result<(), Error> {
        let composer_custom_types = self.custom_types.borrow();

        for (workflow_index, workflow) in self.workflows.borrow().iter().enumerate() {
            if workflow.tasks.is_empty() {
                continue;
            }

            let workflow_name = format!("{}_{}", workflow.name, workflow.version);
            let types_rs = generate_types_rs_file_code(
                &self.workflows.borrow()[workflow_index],
                &composer_custom_types,
            )?;
            let temp_dir =
                self.copy_boilerplate(build_path, &types_rs, workflow_name.clone(), &workflow)?;

            self.build(quiet, &temp_dir)?;

            let wasm_path = format!(
                "{}/boilerplate/target/wasm32-wasi/release/boilerplate.wasm",
                temp_dir.display().to_string()
            );

            fs::create_dir_all(out_path.join("output"))?;

            fs::copy(
                wasm_path,
                &out_path
                    .join("output")
                    .join(format!("{workflow_name}.wasm")),
            )?;

            fs::remove_dir_all(temp_dir)?;
        }
        Ok(())
    }
}
