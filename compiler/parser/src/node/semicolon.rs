use crate::compiler::{diagnostics::DiagnosticResult, lexer::token::TokenKind, parser::Parser};

impl Parser {
    pub fn skip_semicolons(&mut self) -> DiagnosticResult {
        while self.next_if_eq(TokenKind::SemiColon)?.is_some() {}
        Ok(())
    }
}
