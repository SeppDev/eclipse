use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Fields, RawField},
};
impl Tokens {
    pub fn parse_fields(&mut self) -> CompileResult<Fields> {
        let info = self.expect_tokens(vec![Token::StartScope, Token::OpenParen], false)?;

        if let Token::StartScope = info.token {
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
            return Ok(Fields::Struct(fields));
        } else {
            let mut fields = Vec::new();
            while self
                .peek_expect_tokens(vec![Token::EndScope], true)
                .is_none()
            {
                self.start_next();
                let data_type = self.parse_type()?;
                fields.push(data_type)
            }
            return Ok(Fields::List(fields));
        }
    }
}
