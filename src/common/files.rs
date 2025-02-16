use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Default)]
pub struct Files {
    files: HashMap<PathBuf, String>,
}
impl Files {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn remove(&mut self, full_path: &PathBuf) -> Option<String> {
        return self.files.remove(full_path);
    }
    pub fn from_cache(&self, full_path: &PathBuf) -> Option<&String> {
        self.files.get(full_path)
    }
    pub fn read_or_cache(&mut self, full_path: &PathBuf) -> Option<&String> {
        if self.files.contains_key(full_path) {
            return self.from_cache(full_path);
        }
        let source = match fs::read_to_string(full_path) {
            Ok(s) => s,
            Err(_) => return None
        };

        let _old = self.files.insert(full_path.clone(), source);
        self.from_cache(full_path)
    }
}
