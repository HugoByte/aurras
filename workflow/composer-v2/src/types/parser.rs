use super::SourceFiles;

pub trait Parser {
    fn parse(&self, source: &SourceFiles);
}
