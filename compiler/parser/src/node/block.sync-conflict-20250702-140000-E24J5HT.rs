use diagnostics::DiagnosticResult;
use syntax::{ast::*, token::TokenKind};

use super::super::Parser;

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> DiagnosticResult<RawNode> {
        let mut body = Vec::new();

        while self.next_if_eq(TokenKind::CloseCurlyBracket)?.is_none() {
            let node = self.expect_node()?;
            body.push(node);
        }

        Ok(RawNode::Block(body))
    }
    pub fn parse_while(&mut self) -> DiagnosticResult<RawNode> {
        let condition = self.expect_expression()?.into();
        let body = self.expect_expression()?.into();

        Ok(RawNode::While { condition, body })
    }
    pub fn parse_loop(&mut self) -> DiagnosticResult<RawNode> {
        let body = self.expect_expression()?.into();
        Ok(RawNode::Loop(body))
    }
}
