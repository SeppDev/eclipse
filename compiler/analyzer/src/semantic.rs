use std::path::PathBuf;

use syntax::{ast, hlir};

use crate::Analyzer;

impl Analyzer<'_> {
    pub fn analyze(&mut self, collection: ast::ModuleCollection) -> hlir::ModuleCollection {
        let modules: Vec<hlir::Module> = collection
            .modules
            .into_iter()
            .map(|(path, module)| self.module(path, module))
            .collect();

        hlir::ModuleCollection { modules }
    }
    fn module(&mut self, relative_path: PathBuf, module: ast::Module) -> hlir::Module {
        let (imports, nodes) = self.extract_imports(module.nodes);
        let nodes = nodes.into_iter().map(|n| self.node(n)).collect();
        hlir::Module { imports, nodes }
    }
    fn extract_imports(&mut self, nodes: Vec<ast::Node>) -> (Vec<String>, Vec<ast::Node>) {
        let mut imports = Vec::new();

        let nodes = nodes
            .into_iter()
            .filter_map(|n| {
                let path = match n.raw {
                    ast::RawNode::Import(path) => path.raw,
                    _ => return Some(n),
                };
                imports.push(path);
                None
            })
            .collect();

        (imports, nodes)
    }
    fn node(&mut self, node: ast::Node) -> hlir::Node {
        use ast::RawNode;

        match node.raw {
            RawNode::Integer(i) => hlir::Node::Integer(i),
            RawNode::Return(_) => hlir::Node::Return(None),
            RawNode::Block(body) => {
                hlir::Node::Block(body.into_iter().map(|n| self.node(n)).collect())
            }
            RawNode::Function {
                name,
                parameters,
                return_type,
                node,
            } => hlir::Node::Function {
                name: name.raw,
                parameters: Vec::new(),
                return_type: hlir::Type::Void,
                node: Box::new(self.node(*node)),
            },
            r => todo!("{r:?}"),
        }
    }
}
