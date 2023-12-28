use clap::StructOpt;

use crate::types::Context;
use std::path::PathBuf;

use super::*;

/// Compile the config file.
#[derive(StructOpt, Debug)]
pub struct Validate {
    #[structopt(
        long,
        help = "Optional path for the build directory",
        parse(from_os_str)
    )]
    pub build_directory: Option<PathBuf>,

    pub source: Option<PathBuf>,
}

impl Execute<Context> for Validate {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        context.init(self.source, self.build_directory, None)?;
        context.parse()?;
        Ok(())
    }
}
