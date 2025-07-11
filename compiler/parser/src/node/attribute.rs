use diagnostics::DiagnosticResult;
use lexer::token::TokenKind::*;
use syntax::ast::{RawAttribute, RawNode};

use crate::Parser;

impl Parser {
    pub fn parse_attribute(&mut self) -> DiagnosticResult<RawNode> {
        let start = self.start();
        self.expect_single(OpenBracket)?;
        let key = self.expect_identifier()?;
        self.expect_single(CloseBracket)?;

        let raw = RawAttribute::Simple(key.into());
        Ok(RawNode::Attribute(self.located(raw, start)))
    }
}
