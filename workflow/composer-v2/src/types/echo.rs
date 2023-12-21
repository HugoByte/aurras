use composer_primitives::{SourceFiles, result, BuildDirectory, OutputDirectory, constant::{ENTRY_FILE, FILE_EXTENSION}};
use echo_library::Composer;
use crate::errors::IOError;

use super::Parser;



impl Parser for Composer {
    fn parse(&self, files: &SourceFiles) -> result::Result<()> {
        match self.compile(&format!("{}.{}", ENTRY_FILE, FILE_EXTENSION), files){
            Ok(_) =>  Ok(()),
            Err(err) => Err(Box::new(IOError::Anyhow(err))),
        }
    }

    fn build(
        &self,
        build_directory: &BuildDirectory,
        output_directory: &OutputDirectory,
        quiet: bool,
    ) {
        self.build_directory(&build_directory.path, &output_directory.base(), quiet);
    }
}
