use std::{collections::HashMap, path::PathBuf};

use super::Node;

#[derive(Debug)]
pub struct Module {
    imports: Vec<PathBuf>,
    body: Vec<Node>,
}
impl Module {
    pub fn new(body: Vec<Node>) -> Self {
        Self {
            body,
            imports: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Modules {
    files: HashMap<PathBuf, Module>,
}
impl Modules {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, path: PathBuf, module: Module) -> Option<Module> {
        self.files.insert(path, module)
    }
    pub fn get(&self, path: &PathBuf) -> Option<&Module> {
        self.files.get(path)
    }
}
