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

pub struct FileSystemResolver;
impl ModuleResolver for FileSystemResolver {}
impl From<FileSystemResolver> for Resolver {
    fn from(value: FileSystemResolver) -> Self {
        Resolver::FileSystem(value)
    }
}

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
impl From<MockResolver> for Resolver {
    fn from(value: MockResolver) -> Self {
        Resolver::Mock(value)
    }
}
