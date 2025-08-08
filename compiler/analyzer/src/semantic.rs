use std::path::PathBuf;

use diagnostics::DiagnosticResult;
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
    fn node(&mut self, node: ast::Node) -> hlir::Node {
        use ast::RawNode;

        match node.raw {
            RawNode::Integer(i) => hlir::Node::Integer(i),
            RawNode::Bool(value) => hlir::Node::Boolean(value),
            // RawNode::Import(name) =>
            RawNode::Return(expr) => match expr {
                Some(expr) => {
                    let expr = self.node(*expr);
                    hlir::Node::Return(Some(Box::new(expr)))
                }
                None => hlir::Node::Return(None),
            },
            RawNode::Block(body) => {
                hlir::Node::Block(body.into_iter().map(|n| self.node(n)).collect())
            }
            RawNode::Declare {
                mutable,
                name,
                data_type,
                node,
            } => {
                let data_type = self.extract_data_type(&data_type, &node).unwrap();
                let value = Box::new(self.node(*node));

                todo!()
            }
            r => todo!("{r:?}"),
        }
    }
    fn extract_data_type(
        &self,
        expected: &Option<ast::Type>,
        node: &ast::Node,
    ) -> DiagnosticResult<hlir::Type> {
        use ast::RawNode;

        let data_type = match &node.raw {
            RawNode::Integer(_) => hlir::Type::Int(32),
            RawNode::Bool(_) => hlir::Type::Boolean,
            t => todo!("{t:#?}"),
        };

        Ok(data_type)
    }
}
