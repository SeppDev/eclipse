use crate::{common::errors::CompileResult, compiler::lexer::kind::TokenKind};

use super::Reader;

impl Reader {
    pub fn parse_identifer(&mut self) -> CompileResult<Option<TokenKind>> {
        let mut body = String::new();
        let mut start = self.advance().unwrap();
        body.push(start.raw);

        loop {
            let char = match self
                .advance_if(|c| c == &'_' || c.is_ascii_alphabetic() || c.is_ascii_digit())
            {
                Some(c) => c,
                None => break,
            };
            start.position.set_end(char.position.end);
            body.push(char.raw);
        }

        Ok(Some(TokenKind::Identifier(start.position, body)))
    }
}
