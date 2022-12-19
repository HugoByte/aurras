use std::{
    env::current_dir,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::{errors, types::Result, types::FILE_EXTENSION};
use itertools::Either;
use walkdir::WalkDir;
use std::collections::HashSet;
use crate::types::Parser;

#[derive(Clone)]
pub struct SourceFiles {
    base: PathBuf,
    files: HashSet<PathBuf>,
}

impl SourceFiles {
    pub fn new(path: Option<PathBuf>) -> Result<SourceFiles> {
        let mut base = match path {
            Some(path) => {
                if path.is_file() {
                    path.parent().unwrap().to_path_buf()
                } else if path.is_dir() {
                    path
                } else {
                    return Err(Box::new(errors::IOError::PathNotFound));
                }
            }
            None => current_dir().map_err(errors::io_error)?,
        };

        let file_paths = fs::read_dir(&base)
            .map_err(errors::io_error)?
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
