use std::{
    env::current_dir,
    ffi::OsStr,
    fs,
    path::PathBuf,
};
use anyhow::Error;

use itertools::Either;
use walkdir::WalkDir;
use std::collections::HashSet;

use crate::constant::FILE_EXTENSION;

#[derive(Clone, Debug  )]
pub struct SourceFiles {
    base: PathBuf,
    files: HashSet<PathBuf>,
}

impl SourceFiles {
    pub fn new(path: Option<PathBuf>) -> Result<SourceFiles, Error> {
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

        let file_paths = fs::read_dir(&base)
            .unwrap()
            .into_iter()
            .flat_map(|item| {
                let item = item.unwrap();

                if item.path().is_dir() {
                    Either::Left(
                        WalkDir::new(item.path())
                            .into_iter()
                            .filter_map(|e| e.ok())
                            .filter(move |e| {
                                e.path().extension() == Some(OsStr::new(FILE_EXTENSION))
                            })
                            .map(|e| e.into_path()),
                    )
                } else {
                    Either::Right(Box::new(
                        vec![item.path()]
                            .into_iter()
                            .filter(|e| e.extension() == Some(OsStr::new(FILE_EXTENSION))),
                    ))
                }
            })
            .collect::<HashSet<PathBuf>>();

        Ok(SourceFiles {
            base,
            files: file_paths,
        })
    }

    pub fn files(&self) -> &HashSet<PathBuf> {
        &self.files
    }

    pub fn base(&self) -> &PathBuf {
        &self.base
    }
    
}
