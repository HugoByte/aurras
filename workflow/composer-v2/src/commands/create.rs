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
        let current = std::env::current_dir().map_err(|error| io_error(error))?;
        let package = current.join(&self.package_name);
        fs::create_dir_all(&package).map_err(|error| io_error(error))?;

        let temp_path = package.join("main.echo");
        let content = format!(
            "workflows(
                name = {},
                version = \"0.0.1\",
                tasks = []
            )",
            self.package_name
        );

        fs::write(&temp_path, content.as_bytes()).map_err(|error| io_error(error))
    }
}
