use diagnostics::DiagnosticResult;
use syntax::{
    ast::RawNode,
    operators::EqualsOperation,
    token::{Token, TokenKind},
};

use super::super::Parser;

use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn parse_set_operation(&mut self, name: Token) -> DiagnosticResult<RawNode> {
        let name = name.into();
        let info = self.expect(&vec![
            Equals,
            PlusEquals,
            SubtractEquals,
            MultiplyEquals,
            DivideEquals,
            RemainderEquals,
        ])?;

        let operation = match info.kind {
            Equals => EqualsOperation::Equals,
            PlusEquals => EqualsOperation::PlusEquals,
            SubtractEquals => EqualsOperation::SubtractEquals,
            DivideEquals => EqualsOperation::DivideEquals,
            MultiplyEquals => EqualsOperation::MultiplyEquals,
            RemainderEquals => EqualsOperation::RemainderEquals,
            _ => unreachable!(),
        };
        let value = self.expect_expression()?.into();

        Ok(RawNode::SetPath {
            path: name,
            operation,
            value,
        })
    }
}
