use crate::{
    compiler::{
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    fn expect_potential_value(&mut self) -> DiagnosticResult<Option<Box<Node>>> {
        if !self.peek().kind.is_expression_start() {
            return Ok(None);
        }
        let node = self.expect_expression()?;
        Ok(Some(Box::new(node)))
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
