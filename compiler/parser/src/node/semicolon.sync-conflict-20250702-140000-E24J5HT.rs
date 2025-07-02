use diagnostics::DiagnosticResult;
use syntax::token::TokenKind;

use super::super::Parser;

impl<'a> Parser<'a> {
    pub fn skip_semicolons(&mut self) -> DiagnosticResult {
        while self.next_if_eq(TokenKind::SemiColon)?.is_some() {}
        Ok(())
    }
}
