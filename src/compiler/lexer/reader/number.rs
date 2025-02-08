use crate::{
    common::errors::CompileResult,
    compiler::lexer::kind::{LocatedString, TokenKind},
};

use super::Reader;

impl Reader {
    pub fn parse_number(&mut self) -> CompileResult<Option<TokenKind>> {
        let integer = self.parse_integer();

        return integer;
    }
    pub fn parse_integer(&mut self) -> CompileResult<Option<TokenKind>> {
        let mut body = String::new();
        let mut start = self.advance().unwrap();
        body.push(start.raw);

        loop {
            let char = match self.advance_if(|c| c.is_ascii_digit()) {
                Some(c) => c,
                None => break,
            };
            start.position.set_end(char.position.end);
            body.push(char.raw);
        }

        Ok(Some(TokenKind::Integer(LocatedString::new(
            body,
            start.position,
        ))))
    }
}
