use super::{lexer::token::TokenInfo, nodes::ast::Node, CompilerCtx};
use crate::{
    common::position::{Located, Position, PositionRange},
    compiler::lexer::{kind::LocatedString, token::Token},
    diagnostics::{DiagnosticData, DiagnosticResult},
    FILE_EXTENSION,
};
use std::path::PathBuf;

mod node;
mod types;

use reader::TokenReader;
mod reader;

pub struct Parser {
    pub tokens: TokenReader,
    start: Vec<Position>,
    last_position: Option<PositionRange>,
}
impl Parser {
    pub fn new(reader: TokenReader) -> Self {
        Self {
            tokens: reader,
            start: Vec::new(),
            last_position: None,
        }
    }
    pub fn path(&self) -> PathBuf {
        self.tokens.path()
    }
    pub fn start(&mut self) {
        self.start.push(self.peek().position.start)
    }
    pub fn is_eof(&self) -> bool {
        self.peek().raw == Token::EndOfFile
    }
    pub fn next(&mut self) -> DiagnosticResult<TokenInfo> {
        let token = self.tokens.pop().unwrap();

        if token.raw == Token::EndOfFile {
            return Err(DiagnosticData::new(
                "Expected token got <eof>",
                self.path(),
                "",
                token.position,
            ));
        }
        self.last_position = Some(token.position);

        Ok(token)
    }
    pub fn peek(&self) -> &TokenInfo {
        self.tokens.last().unwrap()
    }
    pub fn next_if(
        &mut self,
        func: impl FnOnce(&TokenInfo) -> bool,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        let peeked = self.peek();
        if func(peeked) {
            return Ok(Some(self.next()?));
        }
        Ok(None)
    }
    pub fn next_if_eq(&mut self, value: Token) -> DiagnosticResult<Option<TokenInfo>> {
        self.next_if(|t| t.raw.better_eq(&value))
    }
    pub fn next_if_expected(
        &mut self,
        expected: Vec<Token>,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        let peeked = self.peek();
        for t in expected {
            if t.better_eq(&peeked.raw) {
                return Ok(Some(self.next()?));
            }
        }
        Ok(None)
    }
    pub fn peek_expect(&self, expected: Vec<Token>) -> DiagnosticResult<&TokenInfo> {
        let peeked = self.peek();
        for t in expected.iter() {
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
            self.path(),
            "",
            peeked.position.clone(),
        ))
    }
    pub fn expect(&mut self, expected: Vec<Token>) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(expected)?;
        self.next()
    }
    pub fn expect_single(&mut self, expected: Token) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(vec![expected]);
        self.next()
    }
    pub fn peek_expect_single(&mut self, expected: Token) -> DiagnosticResult<&TokenInfo> {
        self.peek_expect(vec![expected])
    }
    pub fn expect_identifier(&mut self) -> DiagnosticResult<LocatedString> {
        let info = self.expect_single(Token::Identifier(String::new()))?;
        if let Token::Identifier(s) = info.raw {
            return Ok(LocatedString::new(s, info.position));
        }
        unreachable!();
    }
}

