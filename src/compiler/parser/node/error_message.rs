use crate::compiler::{lexer::token::TokenKind, parser::Parser};

impl Parser {
    pub fn generate_error_title(kind: &TokenKind) -> String {
        format!(", got: '{kind:?}'")
    }
}
