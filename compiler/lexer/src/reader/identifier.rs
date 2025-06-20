use diagnostics::DiagnosticResult;

use crate::kind::{LexerKind, LocatedString};

use super::Reader;

impl Reader {
    pub fn parse_identifer(&mut self) -> DiagnosticResult<Option<LexerKind>> {
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

        Ok(Some(LexerKind::Identifier(LocatedString::new(
            body,
            start.position,
        ))))
    }
}
