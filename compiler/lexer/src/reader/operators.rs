use crate::compiler::{diagnostics::DiagnosticResult, lexer::kind::LexerKind};

use super::LexerReader;

impl LexerReader {
    pub fn parse_operators(&mut self) -> DiagnosticResult<Option<LexerKind>> {
        let mut operators = Vec::new();
        operators.push(self.advance().unwrap());

        loop {
            let char =
                match self.advance_if(|c| c.is_ascii_punctuation() && !(c == &'"' || c == &'\'')) {
                    Some(c) => c,
                    None => break,
                };
            operators.push(char);
        }

        Ok(Some(LexerKind::Operators(operators)))
    }
}
