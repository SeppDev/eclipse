use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Layout, RawField, RawLayout},
};
impl Tokens {
    pub fn parse_struct(&mut self) -> CompileResult<Layout> {
        let name = self.parse_identifier()?;
        let fields = self.parse_fields()?;
        
        return Ok(self.create_located(RawLayout::Struct { name, fields }));
    }
}
