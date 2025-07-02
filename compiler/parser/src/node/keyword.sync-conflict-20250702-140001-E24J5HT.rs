use syntax::ast::{Node, RawNode};
use diagnostics::DiagnosticResult;

use super::super::Parser;

impl<'a> Parser<'a> {

    fn expect_potential_value(&mut self) -> DiagnosticResult<Option<Box<Node>>> {
        match self.get_expression()? {
            Some(e) => Ok(Some(Box::new(e))),
            None => Ok(None),
        }
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
