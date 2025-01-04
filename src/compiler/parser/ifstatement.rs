use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, RawNode},
};

impl Tokens {
    pub fn parse_ifstatement(&mut self) -> CompileResult<Node> {
        let expression = self.parse_expression(true)?.unwrap();
        self.expect_tokens(vec![Token::StartScope], false)?;

        let body = self.parse_body()?;

        let else_body = if self.peek_expect_tokens(vec![Token::Else], true).is_some() {
            self.expect_tokens(vec![Token::StartScope], false)?;
            let body = self.parse_body()?;
            Some(body)
        } else {
            None
        };

        return Ok(self.create_located(RawNode::IfStatement {
            expression,
            body,
            elseif: Vec::new(),
            else_body,
        }));
    }
}
