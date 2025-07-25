use super::Node;

#[derive(Debug)]
pub struct Module {
    pub imports: Vec<String>,
    pub nodes: Vec<Node>,
}
impl Module {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes,
            imports: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ModuleCollection {
    pub modules: Vec<Module>,
}
