use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

impl Tokens {
    pub fn parse_struct(&mut self) -> CompileResult<NodeInfo> {
        let name = self.parse_identifier()?;
        let _ = self.expect_tokens(vec![Token::StartScope], false);
        let mut fields = Vec::new();

        if self
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_some()
        {
            return Ok(self.create_node(Node::Struct { name, fields }));
        };

        while self
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_none()
        {
            let name = self.parse_identifier()?;
            let data_type = self.parse_type()?;
            fields.push((name, data_type));
        }

        return Ok(self.create_node(Node::Struct { name, fields }));
    }
}
