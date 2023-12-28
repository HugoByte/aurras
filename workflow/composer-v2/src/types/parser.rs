use super::*;
use composer_primitives::{BuildDirectory, OutputDirectory, SourceFiles};

pub trait Parser {
    fn parse(&self, source: &SourceFiles) -> Result<()>;
    fn build(&self, build_directory: &BuildDirectory, output_directory: &OutputDirectory, quiet: bool)-> Result<()>;
}
