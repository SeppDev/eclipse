use diagnostics::{DiagnosticData, DiagnosticResult};
use syntax::ast;

use super::Analyzer;

impl Analyzer {
    pub fn semantics(&self, body: &Vec<ast::Node>) -> DiagnosticResult {
        for node in body {
            use ast::RawNode::*;
            match &node.raw {
                Function { node, .. } => self.expression(node)?,
                Return(node) => match node {
                    Some(node) => self.expression(node)?,
                    None => continue,
                },
                _ => return DiagnosticData::error().to_err(),
            };
        }

        Ok(())
    }
    pub fn expression(&self, node: &ast::Node) -> DiagnosticResult {
        use ast::RawNode::*;

        match &node.raw {
            Integer(_) => Ok(()),
            _ => {
                return DiagnosticData::error()
                    .title("Expected expression")
                    .to_err();
            }
        }
    }
}
