use std::{iter::Peekable, vec::IntoIter};

use crate::{
    common::position::Located,
    compiler::lexer::token::{Token, TokenInfo}, diagnostics::DiagnosticResult,
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
    pub fn expect(&mut self, expected: &Vec<Token>) -> DiagnosticResult<TokenInfo> {
        let peeked = self.peek().unwrap();
        for t in expected {
            if t.better_eq(&peeked.raw) {
                return Ok(self.next().unwrap());
            }
        }
        todo!()
    }
    pub fn expect_identifier(&mut self) -> DiagnosticResult<Located<String>> {
        let info = self.expect(&vec![Token::Identifier(String::new())])?;
        if let Token::Identifier(s) = info.raw {
            Ok(Located::new(s, info.position))
        } else {
            panic!()
        }
    }
}
