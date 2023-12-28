use std::fs;

use clap::StructOpt;
use composer_primitives::result;

use crate::errors::io_error;

#[derive(StructOpt, Debug)]
pub struct Create {
    pub package_name: String,
}

impl Create {
    pub fn execute(self) -> result::Result<()> {
        let current = std::env::current_dir().map_err(io_error)?;
        let package = current.join(&self.package_name);
        fs::create_dir_all(&package).map_err(io_error)?;

        let temp_path = package.join("main.echo");
        let content = format!(
            "\
hello_world = task(
    kind = \"hello_world\",
    action_name = \"hello_world\",
    input_arguments = [
        argument(
            name=\"name\",
            input_type = String,
            default_value = \"World\"
        ),
    ],
)

workflows(
    name = \"{}\",
    version = \"0.0.1\",
    tasks = [hello_world]
)",
            self.package_name
        );

        fs::write(temp_path, content.as_bytes()).map_err(io_error)
    }
}
