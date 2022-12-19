use crate::types::{Context, Result};
use aurras_primitives::Execute;
use clap::StructOpt;
use std::path::PathBuf;

/// Compile and build program command.
#[derive(StructOpt, Debug)]
pub struct Build {
    #[structopt(
        long,
        help = "Optional path for the build directory",
        parse(from_os_str)
    )]
    pub build_directory: Option<PathBuf>,

    pub source: Option<PathBuf>,
}

impl Execute<Context> for Build {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        context.init(self.source, self.build_directory, None)?;
        context.parse();
        // Initialize

        Ok(())
    }
}
