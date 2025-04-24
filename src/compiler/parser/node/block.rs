use crate::compiler::{
    diagnostics::DiagnosticResult,
    lexer::token::TokenKind,
    nodes::ast::{Node, RawNode},
    parser::Parser,
};

impl Parser {
    pub fn parse_block(&mut self) -> DiagnosticResult<RawNode> {
        let mut body = Vec::new();

        loop {
            if self.next_if_eq(TokenKind::CloseBlock)?.is_some() {
                break;
            }

            let node = self.expect_node()?;
            body.push(node);
        }

        Ok(RawNode::Block(body))
    }
    pub fn expect_block(&mut self) -> DiagnosticResult<Node> {
        self.peek_expect_single(TokenKind::OpenBlock)?;
        self.expect_base_expression()
    }
}
