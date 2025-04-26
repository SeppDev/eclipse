use std::{collections::HashMap, path::PathBuf};

use crate::compiler::Path;

#[derive(Default)]
pub struct Files {
    files: HashMap<Path, String>,
}
impl Files {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cache(&mut self, full_path: Path, source: String) {
        self.files.insert(full_path, source);
    }
    pub fn from_cache(&self, full_path: &Path) -> Option<&String> {
        self.files.get(full_path)
    }
    pub fn fs_read(&self, full_path: &PathBuf) -> std::io::Result<String> {
        std::fs::read_to_string(full_path)
    }
}
