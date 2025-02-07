use crate::{common::errors::CompileResult, compiler::lexer::kind::TokenKind};

use super::Reader;

impl Reader {
    pub fn next_string(&mut self) -> CompileResult<Option<TokenKind>> {
        todo!()
    }
}
