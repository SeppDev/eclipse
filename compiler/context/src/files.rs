use std::{collections::HashMap, path::PathBuf};

pub trait ResolveFile {
    fn read(&self, path: &PathBuf) -> Option<String>;
    fn write(&mut self, path: &PathBuf, contents: &str);
}

pub enum FileResolver {
    Mock(MockResolver),
    FileSystem(FileSystemResolver),
}
impl Default for FileResolver {
    fn default() -> Self {
        Self::FileSystem(FileSystemResolver::default())
    }
}

impl ResolveFile for FileResolver {
    fn read(&self, path: &PathBuf) -> Option<String> {
        use FileResolver::*;
        match self {
            Mock(rs) => rs.read(path),
            FileSystem(rs) => rs.read(path),
        }
    }
    fn write(&mut self, path: &PathBuf, contents: &str) {
        use FileResolver::*;
        match self {
            Mock(rs) => rs.write(path, contents),
            FileSystem(rs) => rs.write(path, contents),
        }
    }
}

#[derive(Default)]
pub struct FileSystemResolver;
impl ResolveFile for FileSystemResolver {
    fn read(&self, path: &PathBuf) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
    fn write(&mut self, path: &PathBuf, contents: &str) {
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();
        std::fs::write(path, contents).unwrap()
    }
}
impl From<FileSystemResolver> for FileResolver {
    fn from(value: FileSystemResolver) -> Self {
        FileResolver::FileSystem(value)
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
impl ResolveFile for MockResolver {
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
impl From<MockResolver> for FileResolver {
    fn from(value: MockResolver) -> Self {
        FileResolver::Mock(value)
    }
}
