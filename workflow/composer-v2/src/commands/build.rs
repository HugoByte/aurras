use crate::types::Context;
use clap::StructOpt;
use std::path::PathBuf;

use super::*;
/// Compile and build program command.
#[derive(StructOpt, Debug)]
pub struct Build {
    #[structopt(
        short,
        long,
        help = "Optional path for the build directory",
        parse(from_os_str)
    )]
    pub build_directory: Option<PathBuf>,

    #[structopt(
        short,
        long,
        help = "Optional path to output workflow wasm",
        parse(from_os_str)
    )]
    pub output: Option<PathBuf>,

    #[structopt(
        short,
        long,
        help = "Optional path to config files",
        parse(from_os_str)
    )]
    pub source: Option<PathBuf>,
}

impl Execute<Context> for Build {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        context.init(self.source, self.build_directory, self.output)?;
        context.parse()?;
        context.build()?;

        Ok(())
    }
}
