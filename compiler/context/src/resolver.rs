use std::{collections::HashMap, path::PathBuf};

pub trait ModuleResolver {
    fn read(&self, path: &PathBuf) -> Option<String>;
    fn write(&mut self, path: &PathBuf, contents: &str);
}

pub enum Resolver {
    Mock(MockResolver),
    FileSystem(FileSystemResolver),
}
impl Default for Resolver {
    fn default() -> Self {
        Self::FileSystem(FileSystemResolver::default())
    }
}
impl ModuleResolver for Resolver {
    fn read(&self, path: &PathBuf) -> Option<String> {
        use Resolver::*;
        match self {
            Mock(rs) => rs.read(path),
            FileSystem(rs) => rs.read(path),
        }
    }
    fn write(&mut self, path: &PathBuf, contents: &str) {
        use Resolver::*;
        match self {
            Mock(rs) => rs.write(path, contents),
            FileSystem(rs) => rs.write(path, contents),
        }
    }
}

#[derive(Default)]
pub struct FileSystemResolver;
impl ModuleResolver for FileSystemResolver {
    fn read(&self, path: &PathBuf) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
    fn write(&mut self, path: &PathBuf, contents: &str) {
        let _ = std::fs::write(path, contents);
    }
}
impl From<FileSystemResolver> for Resolver {
    fn from(value: FileSystemResolver) -> Self {
        Resolver::FileSystem(value)
    }
}

#[derive(Default)]
pub struct MockResolver {
    files: HashMap<PathBuf, String>,
}
impl MockResolver {
    pub fn new() -> Self {
        Self::default()
    }
}
impl ModuleResolver for MockResolver {
    fn read(&self, path: &PathBuf) -> Option<String> {
        match self.files.get(path) {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }
    fn write(&mut self, path: &PathBuf, contents: &str) {
        self.files.insert(path.clone(), contents.to_string());
    }
}
impl From<MockResolver> for Resolver {
    fn from(value: MockResolver) -> Self {
        Resolver::Mock(value)
    }
}
