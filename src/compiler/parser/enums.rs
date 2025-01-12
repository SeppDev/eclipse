use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Layout, RawLayout},
};

impl Tokens {
    pub fn parse_enum(&mut self) -> CompileResult<Layout> {
        let name = self.parse_identifier()?;
        let mut enums = Vec::new();
        
        let mut fields = Vec::new();
        while self
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_none()
        {
            self.start_next();
            let identifier = self.parse_type()?;
            let fields = if self.peek_expect_tokens(vec![Token::StartScope, Token::OpenParen], false).is_some() {
                Some(self.parse_fields()?)
            } else {
                None
            };
            
            enums.push((identifier, fields))
        }

        return Ok(self.create_located(RawLayout::Enum { name, fields }));
    }
}
