use std::path::PathBuf;

use crate::compiler::lexer::token::TokenInfo;

type Tokens = Vec<TokenInfo>;

#[derive(Debug)]
pub struct TokenReader {
    file: PathBuf,
    tokens: Tokens,
}
impl TokenReader {
    pub fn new(mut tokens: Tokens, file: PathBuf) -> Self {
        tokens.reverse();
        Self { tokens, file }
    }
    pub fn pop(&mut self) -> Option<TokenInfo> {
        self.tokens.pop()
    }
    pub fn last(&self) -> Option<&TokenInfo> {
        self.tokens.last()
    }
    pub fn path(&self) -> PathBuf {
        self.file.clone()
    }
}
