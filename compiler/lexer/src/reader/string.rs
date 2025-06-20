use diagnostics::DiagnosticResult;

use crate::kind::{LexerKind, LocatedString};

use super::Reader;

impl Reader {
    pub fn parse_string(&mut self) -> DiagnosticResult<Option<LexerKind>> {
        let mut body = String::new();
        let mut delimiter = self.advance().unwrap();
        let is_string = delimiter.raw == '"';

        loop {
            let char = match self.advance_if(|c| c != &delimiter.raw) {
                Some(c) => c,
                None => break,
            };

            delimiter.position.set_end(char.position.end);
            body.push(char.raw);
        }
        self.advance();

        let string = LocatedString::new(body, delimiter.position);

        Ok(Some(if is_string {
            LexerKind::String(string)
        } else {
            LexerKind::Character(string)
        }))
    }
}
