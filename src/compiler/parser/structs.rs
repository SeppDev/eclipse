use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Node, RawField, RawNode},
};
impl Tokens {
    pub fn parse_struct(&mut self) -> CompileResult<Node> {
        let name = self.parse_identifier()?;
        let _ = self.expect_tokens(vec![Token::StartScope], false);
        let mut fields = Vec::new();

        while self
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_none()
        {
            self.start_next();
            let name = self.parse_identifier()?;
            let data_type = self.parse_type()?;
            fields.push(self.create_located(RawField { name, data_type }));
        }

        return Ok(self.create_located(RawNode::Struct { name, fields }));
    }
}
