use diagnostics::DiagnosticResult;
use syntax::ast::{Node, RawNode};

use crate::Parser;

impl Parser {
    pub fn parse_attribute(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Return(self.expect_potential_value()?))
    }
}
