use std::path::PathBuf;

use common::position::Span;
use diagnostics::DiagnosticResult;
use syntax::{ast, hir};

use crate::Analyzer;

impl Analyzer<'_> {
    pub fn analyze(&mut self, collection: ast::ModuleCollection) -> hir::ModuleCollection {
        let modules: Vec<hir::Module> = collection
            .modules
            .into_iter()
            .map(|(path, module)| self.module(path, module))
            .collect();

        hir::ModuleCollection { modules }
    }
    fn module(&mut self, relative_path: PathBuf, module: ast::Module) -> hir::Module {
        use ast::RawNode;

        for node in module.nodes {
            match node.raw {
                RawNode::Function {
                    name,
                    parameters,
                    return_type,
                    node,
                } => self.function(name, parameters, return_type, *node),
                r => todo!("{r:?}"),
            };
        }

        todo!()
    }
    fn function(
        &mut self,
        name: Span<String>,
        parameters: Vec<ast::Parameter>,
        return_type: ast::Type,
        body: ast::Node,
    ) -> hir::Function {
        use ast::RawNode;

        todo!()
    }
    fn node(&mut self, node: ast::Node, expected: &Option<ast::Type>) -> hir::Node {
        use ast::RawNode;

        match node.raw {
            RawNode::Integer(i) => hir::Node::Integer(i),
            RawNode::Bool(value) => hir::Node::Boolean(value),
            RawNode::Return(expr) => match expr {
                Some(expr) => {
                    let expr = self.node(*expr, &None);
                    hir::Node::Return(Some(Box::new(expr)))
                }
                None => hir::Node::Return(None),
            },
            RawNode::Block(body) => {
                hir::Node::Block(body.into_iter().map(|n| self.node(n, &None)).collect())
            }
            RawNode::Declare {
                mutable,
                name,
                data_type,
                node,
            } => {
                let data_type = self.extract_data_type(&data_type, &node).unwrap();
                let value = Box::new(self.node(*node, &None));

                todo!()
            }
            r => todo!("{r:?}"),
        }
    }
    fn extract_data_type(
        &self,
        expected: &Option<ast::Type>,
        node: &ast::Node,
    ) -> DiagnosticResult<hir::Type> {
        use ast::RawNode;

        let data_type = match &node.raw {
            RawNode::Integer(_) => hir::Type::Int(32),
            RawNode::Bool(_) => hir::Type::Boolean,
            t => todo!("{t:#?}"),
        };

        Ok(data_type)
    }
}
