mod cache;

use std::{collections::HashMap, path::PathBuf};

use super::errors::CompileResult;

pub const FILE_EXTENSION: &str = "ecl";

#[derive(Debug)]
pub struct ProjectFiles {
    pub(super) project_path: PathBuf,
    pub(super) src: Files,
}
impl ProjectFiles {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project_path,
            src: Files::new(),
        }
    }
    pub fn pre_cache(&mut self) -> CompileResult<()> {
        self.src.pre_cache(&self.project_path)?;

        Ok(())
    }

    pub fn read(&self, relative_path: &PathBuf) -> Option<&File> {
        if relative_path.starts_with("src/") {
            return self.src.read(&self.project_path.join(relative_path));
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct Files {
    files: HashMap<PathBuf, File>,
}
impl Files {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(&self, full_path: &PathBuf) -> Option<&File> {
        self.files.get(full_path)
    }
}

#[derive(Debug)]
pub struct File {
    pub body: String,
}
impl From<String> for File {
    fn from(value: String) -> Self {
        File { body: value }
    }
}
