use std::borrow::Borrow;

use common::position::{LocatedAt, Position, PositionRange};
use diagnostics::{DiagnosticData, DiagnosticResult};
use lexer::token::{Token, TokenKind};

use crate::Parser;

impl Parser {
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
    pub fn next(&mut self) -> DiagnosticResult<Token> {
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
    pub fn peek(&self) -> &Token {
        self.tokens.last().unwrap()
    }
    pub fn next_if(
        &mut self,
        func: impl FnOnce(&Token) -> bool,
    ) -> DiagnosticResult<Option<Token>> {
        let peeked = self.peek();
        if func(peeked) {
            return Ok(Some(self.next()?));
        }
        Ok(None)
    }
    pub fn next_if_eq(&mut self, kind: impl Borrow<TokenKind>) -> DiagnosticResult<Option<Token>> {
        self.next_if(|t| &t.kind == kind.borrow())
    }
    pub fn peek_expect(&self, expected: &Vec<TokenKind>) -> DiagnosticResult<&Token> {
        let peeked = self.peek();
        for t in expected.iter() {
            if &peeked.kind == t {
                return Ok(peeked);
            }
        }

        let title = format!(
            "Expected token(s): {}, got: {:?}",
            expected
                .iter()
                .map(|e| format!("'{e:?}'"))
                .collect::<Vec<String>>()
                .join(", "),
            &peeked.kind,
        );

        DiagnosticData::error()
            .title(title)
            .position(peeked.position.clone())
            .to_err()
    }
    pub fn expect(&mut self, expected: &Vec<TokenKind>) -> DiagnosticResult<Token> {
        self.peek_expect(&expected)?;
        self.next()
    }
    pub fn expect_single(&mut self, expected: TokenKind) -> DiagnosticResult<Token> {
        self.peek_expect(&vec![expected])?;
        self.next()
    }

    pub fn expect_identifier(&mut self) -> DiagnosticResult<Token> {
        self.expect_single(TokenKind::Identifier)
    }
}
