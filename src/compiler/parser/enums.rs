use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Identifier, Node, RawNode},
};

impl Tokens {
    pub fn parse_enum(&mut self) -> CompileResult<Node> {
        let name = self.parse_identifier()?;
        let _ = self.expect_tokens(vec![Token::StartScope], false);
        let mut fields: Vec<Identifier> = Vec::new();

        loop {
            let identifier = self.parse_identifier()?;
            fields.push(identifier);

            let result = self.expect_tokens(vec![Token::Comma, Token::EndScope], false)?;
            match result.token {
                Token::Comma => continue,
                Token::EndScope => break,
                _ => panic!(),
            }
        }

        return Ok(self.create_located(RawNode::Enum { name, fields }));
    }
}
