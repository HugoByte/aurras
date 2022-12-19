use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::{
    errors,
    types::{BuildDirectory, OutputDirectory, Result, SourceFiles, Parser, Echo},
};

pub(crate) struct Context {
    build_directory: Option<BuildDirectory>,
    output_directory: Option<OutputDirectory>,
    source_files: Option<SourceFiles>,
    parser: Box<dyn Parser>
}

impl Default for Context {
    fn default() -> Self {
        Context { build_directory: None, output_directory: None, source_files: None, parser: Box::new(Echo{}) }
    }
}

impl Context {
    pub fn new() -> Result<Context> {
        Ok(Context::default())
    }

    pub fn init(
        &mut self,
        source: Option<PathBuf>,
        build_directory: Option<PathBuf>,
        output_directory: Option<PathBuf>,
    ) -> Result<()> {
        self.build_directory = Some(BuildDirectory::new(build_directory)?);
        self.source_files = Some(SourceFiles::new(source)?);
        self.output_directory = Some(OutputDirectory::default());
        
        Ok(())
    }

    pub fn parse(&self) {
        &self.parser.parse(&self.source_files.as_ref().unwrap());
    }
}
