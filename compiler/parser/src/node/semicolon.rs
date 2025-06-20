use common::{
    layout::ast::{RawNode, UsePath},
    lexer::token::TokenKind,
    path::Path,
};
use diagnostics::DiagnosticResult;

use crate::Parser;


impl Parser {
    pub fn skip_semicolons(&mut self) -> DiagnosticResult {
        while self.next_if_eq(TokenKind::SemiColon)?.is_some() {}
        Ok(())
    }
}
