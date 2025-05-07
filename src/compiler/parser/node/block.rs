use crate::compiler::{
    ast::RawNode, diagnostics::DiagnosticResult, lexer::token::TokenKind, parser::Parser,
};

impl Parser {
    pub fn parse_block(&mut self) -> DiagnosticResult<RawNode> {
        let mut body = Vec::new();

        while self.next_if_eq(TokenKind::CloseCurlyBracket)?.is_none() {
            let node = self.expect_node()?;
            body.push(node);
        }

        Ok(RawNode::Block(body))
    }
}
