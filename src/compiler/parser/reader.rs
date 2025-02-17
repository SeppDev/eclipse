use std::{iter::Peekable, os::unix::fs::DirEntryExt, path::PathBuf, vec::IntoIter};

use crate::{
    common::position::Located,
    compiler::lexer::{
        kind::LocatedString,
        token::{Token, TokenInfo},
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

type Tokens = Vec<TokenInfo>;

pub(super) struct TokenReader {
    file: PathBuf,
    tokens: Tokens,
}
impl TokenReader {
    pub fn new(mut tokens: Tokens, file: PathBuf) -> Self {
        tokens.reverse();
        Self { tokens, file }
    }
    pub fn next(&mut self) -> TokenInfo {
        self.tokens.pop().unwrap()
    }
    pub fn peek(&self) -> &TokenInfo {
        self.tokens.last().unwrap()
    }
    pub fn next_if(&mut self, func: impl FnOnce(&TokenInfo) -> bool) -> Option<TokenInfo> {
        let peeked = self.peek();
        if func(peeked) {
            return Some(self.next());
        }
        None
    }
    pub fn next_if_eq(&mut self, value: &Token) -> TokenInfo {
        self.next_if(|t| t.raw.better_eq(value)).unwrap()
    }
    pub fn next_if_expected(&mut self, expected: &Vec<Token>) -> Option<TokenInfo> {
        let peeked = self.peek();
        for t in expected {
            if t.better_eq(&peeked.raw) {
                return Some(self.next());
            }
        }
        None
    }
    pub fn peek_expect(&self, expected: &Vec<Token>) -> DiagnosticResult<&TokenInfo> {
        let peeked = self.peek();
        for t in expected {
            if t.better_eq(&peeked.raw) {
                return Ok(peeked);
            }
        }

        Err(DiagnosticData::new(
            format!(
                "Expected token(s): {}, got: '{}'",
                expected
                    .iter()
                    .map(|e| format!("'{e}'"))
                    .collect::<Vec<String>>()
                    .join(", "),
                peeked.raw
            ),
            self.file.clone(),
            "",
            peeked.position.clone(),
        ))
    }
    pub fn expect(&mut self, expected: &Vec<Token>) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(expected)?;
        Ok(self.next())
    }
    pub fn expect_identifier(&mut self) -> DiagnosticResult<LocatedString> {
        let info = self.expect(&vec![Token::Identifier(String::new())])?;
        if let Token::Identifier(s) = info.raw {
            return Ok(LocatedString::new(s, info.position));
        }
        unreachable!()
        // if let Token::Identifier(s) = info.raw {
        // Ok(Located::new(s, info.position))
        // } else {
        // Err(DiagnosticData::new(
        // "Expected identifer",
        // self.file.clone(),
        // "",
        // info.position,
        // ))
        // }
    }
}
