use std::borrow::Borrow;

use crate::{
    common::position::{LocatedAt, Position, PositionRange},
    compiler::{
        diagnostics::{DiagnosticData, DiagnosticResult},
        lexer::token::{TokenInfo, TokenKind},
    },
};

use super::{Parser, ast};

impl Parser {
    pub fn parse(&mut self) -> DiagnosticResult<Vec<ast::Node>> {
        let mut nodes = Vec::new();

        loop {
            if self.is_eof() {
                break;
            }

            let node = self.top_level_expect()?;
            nodes.push(node);
        }

        Ok(nodes)
    }
    pub fn start(&self) -> Position {
        self.peek().position.start
    }
    pub fn located<T>(&mut self, value: T, start: Position) -> LocatedAt<T> {
        let end = self.last_position.end;
        return LocatedAt::new(value, PositionRange::new(start, end));
    }
    pub fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::EndOfFile
    }
    pub fn next(&mut self) -> DiagnosticResult<TokenInfo> {
        let token = self.tokens.pop().unwrap();

        if token.kind == TokenKind::EndOfFile {
            return DiagnosticData::error()
                .title("Expected token got <eof>")
                .position(token.position)
                .to_err();
        }
        self.last_position = token.position;

        Ok(token)
    }
    pub fn peek(&self) -> &TokenInfo {
        self.tokens.last().unwrap()
    }
    pub fn peek_second(&self) -> &TokenInfo {
        self.tokens.get(self.tokens.len() - 2).unwrap()
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
    pub fn next_if_eq(
        &mut self,
        kind: impl Borrow<TokenKind>,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        self.next_if(|t| &t.kind == kind.borrow())
    }
    pub fn peek_expect(&self, expected: &Vec<TokenKind>) -> DiagnosticResult<&TokenInfo> {
        let peeked = self.peek();
        for t in expected.iter() {
            if &peeked.kind == t {
                return Ok(peeked);
            }
        }

        let title = format!(
            "Expected token(s): {}{}",
            expected
                .iter()
                .map(|e| format!("'{e:?}'"))
                .collect::<Vec<String>>()
                .join(", "),
            Self::generate_error_title(&peeked.kind),
        );

        DiagnosticData::error()
            .title(title)
            .position(peeked.position.clone())
            .to_err()
    }
    pub fn expect(&mut self, expected: &Vec<TokenKind>) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&expected)?;
        self.next()
    }
    pub fn expect_single(&mut self, expected: TokenKind) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&vec![expected])?;
        self.next()
    }

    pub fn expect_identifier(&mut self) -> DiagnosticResult<TokenInfo> {
        self.expect_single(TokenKind::Identifier)
    }
}
