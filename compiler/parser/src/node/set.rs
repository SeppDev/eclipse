use common::{
    layout::ast::{RawNode, UsePath},
    lexer::token::TokenKind,
    path::Path,
};
use diagnostics::DiagnosticResult;

use crate::Parser;

use TokenKind::*;

impl Parser {
    pub fn parse_set_operation(&mut self, name: TokenInfo) -> DiagnosticResult<RawNode> {
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
