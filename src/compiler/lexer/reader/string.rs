use crate::{
    common::errors::CompileResult,
    compiler::lexer::kind::{LocatedString, TokenKind},
};

use super::Reader;

impl Reader {
    pub fn parse_string(&mut self) -> CompileResult<Option<TokenKind>> {
        let mut body = String::new();
        let mut delimiter = self.advance().unwrap();
        body.push(delimiter.raw);

        loop {
            let char = match self.advance_if(|c| c != &delimiter.raw) {
                Some(c) => c,
                None => break,
            };
            delimiter.position.set_end(char.position.end);
            body.push(char.raw);
        }

        Ok(Some(TokenKind::String(LocatedString::new(
            body,
            delimiter.position,
        ))))
    }
}
