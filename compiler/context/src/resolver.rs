use std::{collections::HashMap, path::PathBuf};

pub trait ModuleResolver {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

pub enum Resolver {
    Mock(MockResolver),
    FileSystem(FileSystemResolver),
}
impl ModuleResolver for Resolver {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        use Resolver::*;
        match self {
            Mock(rs) => rs.resolve_module(path),
            FileSystem(rs) => rs.resolve_module(path),
        }
    }
}

#[derive(Default)]
pub struct FileSystemResolver;
impl ModuleResolver for FileSystemResolver {}
impl From<FileSystemResolver> for Resolver {
    fn from(value: FileSystemResolver) -> Self {
        Resolver::FileSystem(value)
    }
}

#[derive(Default)]
pub struct MockResolver {
    files: HashMap<PathBuf, &'static str>,
}
impl MockResolver {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, path: PathBuf, body: &'static str) {
        self.files.insert(path, body);
    }
}
impl ModuleResolver for MockResolver {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        match self.files.get(path) {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }
}
impl From<MockResolver> for Resolver {
    fn from(value: MockResolver) -> Self {
        Resolver::Mock(value)
    }
}
