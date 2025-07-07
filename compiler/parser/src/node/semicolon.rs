use diagnostics::DiagnosticResult;
use lexer::token::TokenKind;

use crate::Parser;

impl Parser {
    pub fn skip_semicolons(&mut self) -> DiagnosticResult {
        while self.next_if_eq(TokenKind::SemiColon)?.is_some() {}
        Ok(())
    }
}
