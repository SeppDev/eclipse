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
    pub fn read_or_cache(&mut self, full_path: &PathBuf) -> &String {
        let source = fs::read_to_string(full_path).unwrap();

        if self.files.contains_key(full_path) {
            return self.from_cache(full_path).unwrap();
        }

        self.files.insert(full_path.clone(), source).unwrap();
        self.from_cache(full_path).unwrap()
    }
}
