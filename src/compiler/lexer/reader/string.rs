use crate::{
    compiler::lexer::kind::{LocatedString, TokenKind},
    diagnostics::DiagnosticResult,
};

use super::Reader;

impl Reader {
    pub fn parse_string(&mut self) -> DiagnosticResult<Option<TokenKind>> {
        let mut body = String::new();
        let mut delimiter = self.advance().unwrap();
        body.push(delimiter.raw);

        println!("{:?}", delimiter.raw);

        loop {
            let char = match self.advance_if(|c| c != &delimiter.raw) {
                Some(c) => c,
                None => break,
            };
            delimiter.position.set_end(char.position.end);
            body.push(char.raw);
        }

        let is_string = delimiter.raw == '"';
        let string = LocatedString::new(body, delimiter.position);

        Ok(Some(if is_string {
            TokenKind::String(string)
        } else {
            TokenKind::Character(string)
        }))
    }
}
