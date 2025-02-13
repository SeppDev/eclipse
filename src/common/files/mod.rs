mod cache;

use std::{collections::HashMap, fs, path::PathBuf};

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

    pub fn read(&self, relative_path: &PathBuf) -> CompileResult<Option<&File>> {
        if !relative_path.starts_with("src/") {
            return Ok(None);
        }
        Ok(self.src.read(&self.project_path.join(relative_path)))
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
    // pub fn read_mut(&mut self, full_path: &PathBuf) -> CompileResult<Option<&File>> {
    //     match self.files.get(full_path) {
    //         Some(f) => return Ok(Some(f)),
    //         None => {}
    //     }

    //     if !full_path.exists() {
    //         return Ok(None);
    //     }

    //     let source = fs::read_to_string(full_path)?;

    //     self.files.insert(full_path.clone(), File::from(source));

    //     Ok(self.read(full_path))
    // }
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
