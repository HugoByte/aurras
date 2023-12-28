
use std::{
    env::current_dir,
    path::PathBuf,
};
use anyhow::Error;

#[derive(Clone, Debug )]
pub struct OutputDirectory {
    base: PathBuf,
}

impl OutputDirectory {
    pub fn new(path: Option<PathBuf>) -> Result<OutputDirectory, Error> {
        let base = match path {
            Some(path) => {
                if path.is_file() {
                    path.parent().unwrap().to_path_buf()
                } else if path.is_dir() {
                    path
                } else {
                    return Err(Error::msg("PathNotFound"));
                }
            }
            None => current_dir().unwrap(),
        };

        Ok(OutputDirectory {
            base,
        })
    }

    pub fn base(&self) -> &PathBuf {
        &self.base
    }
}
