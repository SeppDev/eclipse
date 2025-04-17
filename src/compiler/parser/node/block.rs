use crate::{
    compiler::{
        lexer::token::Token,
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_block(&mut self) -> DiagnosticResult<RawNode> {
        let mut body = Vec::new();

        loop {
            if self.next_if_eq(Token::CloseBlock)?.is_some() {
                break;
            }

            let node = self.expect_node()?;
            body.push(node);
        }

        Ok(RawNode::Block(body))
    }
}
