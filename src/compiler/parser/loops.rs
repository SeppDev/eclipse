use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, RawNode},
};

impl Tokens {
    pub fn parse_loop(&mut self) -> CompileResult<Node> {
        let _ = self.expect_tokens(vec![Token::StartScope], false);

        let body = self.parse_body()?;

        return Ok(self.create_located(RawNode::Loop {
            condition: None,
            body,
        }));
    }

    pub fn parse_while(&mut self) -> CompileResult<Node> {
        let expression = self.parse_expression(true)?.unwrap();
        let _ = self.expect_tokens(vec![Token::StartScope], false);

        let body = self.parse_body()?;

        return Ok(self.create_located(RawNode::Loop {
            condition: Some(expression),
            body,
        }));
    }
}
