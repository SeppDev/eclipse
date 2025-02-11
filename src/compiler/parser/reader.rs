use std::{iter::Peekable, vec::IntoIter};

use crate::{
    common::errors::CompileResult,
    compiler::lexer::token::{Token, TokenInfo},
};

type Tokens = Peekable<IntoIter<TokenInfo>>;

pub(super) struct TokenReader {
    tokens: Tokens,
}
impl TokenReader {
    pub fn new(tokens: Tokens) -> Self {
        Self { tokens }
    }
    pub fn next(&mut self) -> Option<TokenInfo> {
        self.tokens.next()
    }
    pub fn next_if(&mut self, func: impl FnOnce(&TokenInfo) -> bool) -> Option<TokenInfo> {
        self.tokens.next_if(func)
    }
    pub fn next_if_eq(&mut self, value: &Token) -> Option<TokenInfo> {
        self.tokens.next_if(|t| t.raw.better_eq(value))
    }
    pub fn peek(&mut self) -> Option<&TokenInfo> {
        self.tokens.peek()
    }
    pub fn expect(&mut self, expected: &Vec<Token>) -> CompileResult<TokenInfo> {
        let peeked = self.peek().unwrap();
        for t in expected {
            if t.better_eq(&peeked.raw) {
                return Ok(self.next().unwrap());
            }
        }
        todo!()
    }
}
