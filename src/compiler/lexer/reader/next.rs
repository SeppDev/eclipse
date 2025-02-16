use crate::{compiler::lexer::kind::TokenKind, diagnostics::DiagnosticResult};

use super::Reader;

impl Reader {
    pub fn next(&mut self) -> DiagnosticResult<Option<TokenKind>> {
        loop {
            let start = match self.peek() {
                Some(c) => c,
                None => return Ok(None),
            };

            return match start.raw {
                '/' if self.peek().unwrap().raw == '*' => {
                    self.read_multi_line_comment();
                    continue;
                }
                '/' if self.peek().unwrap().raw == '/' => {
                    self.read_line_comment();
                    continue;
                }
                '"' | '\'' => self.parse_string(),
                character if character.is_ascii_alphabetic() || character == '_' => {
                    self.parse_identifer()
                }
                character if character.is_ascii_digit() => self.parse_number(),
                character if character.is_ascii_whitespace() => {
                    self.advance();
                    continue;
                }
                character if character.is_ascii_punctuation() => self.parse_operators(),
                character => todo!("Unkown character '{character}'"),
            };
        }
    }
}
