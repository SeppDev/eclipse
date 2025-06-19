use crate::compiler::{
    diagnostics::DiagnosticResult,
    lexer::kind::{LexerKind, LocatedString},
};

use super::LexerReader;

impl LexerReader {
    pub fn parse_number(&mut self) -> DiagnosticResult<Option<LexerKind>> {
        let integer = self.parse_integer()?;

        if match self.peek() {
            Some(char) if char.raw == '.' => match self.peek() {
                Some(char) if char.raw.is_ascii_digit() => false,
                _ => true,
            },
            _ => true,
        } {
            return Ok(Some(LexerKind::Integer(integer)));
        }

        let second = self.parse_integer()?;

        let position = integer.position.start.extend(second.position.end);
        let string = format!("{}.{}", integer.raw, second.raw);
        return Ok(Some(LexerKind::Float(LocatedString::new(string, position))));
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
