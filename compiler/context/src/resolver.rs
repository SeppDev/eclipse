use std::{collections::HashMap, path::PathBuf};

pub trait ModuleResolver {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

pub struct FileSystemResolver;
impl ModuleResolver for FileSystemResolver {}

pub struct MockResolver {
    files: HashMap<PathBuf, String>,
}
impl ModuleResolver for MockResolver {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        match self.files.get(path) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }
}
