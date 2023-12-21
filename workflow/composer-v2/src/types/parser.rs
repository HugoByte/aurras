use composer_primitives::{BuildDirectory, SourceFiles, OutputDirectory};
use super::*;

pub trait Parser {
    fn parse(&self, source: &SourceFiles) -> Result<()>;
    fn build(&self, build_directory: &BuildDirectory, output_directory: &OutputDirectory, quiet: bool);
}
