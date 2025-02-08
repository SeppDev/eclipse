use crate::{common::errors::CompileResult, compiler::lexer::kind::TokenKind};

use super::Reader;

impl Reader {
    pub fn parse_operators(&mut self) -> CompileResult<Option<TokenKind>> {
        let mut operators = Vec::new();
        operators.push(self.advance().unwrap());

        loop {
            let char = match self.advance_if(|c| c.is_ascii_punctuation()) {
                Some(c) => c,
                None => break,
            };
            operators.push(char);
        }

        Ok(Some(TokenKind::Operators(operators)))
    }
}
