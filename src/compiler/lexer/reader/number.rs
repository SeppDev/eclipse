use crate::{
    compiler::lexer::kind::{LocatedString, TokenKind},
    diagnostics::DiagnosticResult,
};

use super::Reader;

impl Reader {
    pub fn parse_number(&mut self) -> DiagnosticResult<Option<TokenKind>> {
        let integer = self.parse_integer()?;

        let dot = self.advance_if_eq('.');
        if !dot.is_some() {
            return Ok(Some(TokenKind::Integer(integer)));
        }

        let char = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        if char.raw.is_ascii_digit() {}

        let second = self.parse_integer()?;

        let position = integer.position.start.extend(second.position.end);
        let string = format!("{}.{}", integer.raw, second.raw);
        return Ok(Some(TokenKind::Float(LocatedString::new(string, position))));
    }
    fn parse_integer(&mut self) -> DiagnosticResult<LocatedString> {
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

        Ok(LocatedString::new(body, start.position))
    }
}
