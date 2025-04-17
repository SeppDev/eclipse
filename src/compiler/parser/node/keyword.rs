use crate::{
    compiler::{
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn expect_potential_value(&mut self) -> DiagnosticResult<Option<Box<Node>>> {
        Ok(match self.expect_potential_node()? {
            Some(n) => Some(Box::new(n)),
            None => None,
        })
    }
    pub fn parse_return(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Return(self.expect_potential_value()?))
    }
    pub fn parse_break(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Break(self.expect_potential_value()?))
    }
    pub fn parse_continue(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Continue(self.expect_potential_value()?))
    }
}
