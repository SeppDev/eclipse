mod cache;

use std::{collections::HashMap, path::PathBuf};

pub const FILE_EXTENSION: &str = "ecl";

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
}

#[derive(Default)]
pub struct Files {
    files: HashMap<PathBuf, File>,
}
impl Files {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct File {}
