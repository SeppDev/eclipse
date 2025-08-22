use std::{collections::HashMap, path::PathBuf};

use super::Node;

#[derive(Debug)]
pub struct Module {
    pub nodes: Vec<Node>,
}
impl Module {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }
}

#[derive(Debug, Default)]
pub struct ModuleCollection {
    pub modules: HashMap<PathBuf, Module>,
}
