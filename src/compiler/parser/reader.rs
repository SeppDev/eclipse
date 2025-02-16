use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

use crate::{
    common::position::Located,
    compiler::lexer::token::{Token, TokenInfo}, diagnostics::{DiagnosticData, DiagnosticResult},
};

type Tokens = Peekable<IntoIter<TokenInfo>>;

pub(super) struct TokenReader {
    file: PathBuf,
    tokens: Tokens,
}
impl TokenReader {
    pub fn new(tokens: Tokens, file: PathBuf,) -> Self {
        Self { tokens, file }
    }
    pub fn next(&mut self) -> Option<TokenInfo> {
        self.tokens.next()
    }
    pub fn next_if(&mut self, func: impl FnOnce(&TokenInfo) -> bool) -> TokenInfo {
        self.tokens.next_if(func).unwrap()
    }
    pub fn next_if_eq(&mut self, value: &Token) -> TokenInfo {
        self.tokens.next_if(|t| t.raw.better_eq(value)).unwrap()
    }
    pub fn peek(&mut self) -> &TokenInfo {
        self.tokens.peek().unwrap()
    }
    pub fn expect(&mut self, expected: &Vec<Token>) -> DiagnosticResult<TokenInfo> {
        let peeked = self.peek();
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
            Err(DiagnosticData::new("Expected identifer", self.file.clone(), "", info.position))
        }
    }
}
