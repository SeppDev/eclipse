use crate::{
    compiler::lexer::{
        kind::LocatedString,
        token::{Token, TokenInfo},
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

use super::Parser;

impl Parser {
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
    pub fn expect_identifier(&mut self) -> DiagnosticResult<LocatedString> {
        let info = self.expect(vec![Token::Identifier(String::new())])?;
        if let Token::Identifier(s) = info.raw {
            return Ok(LocatedString::new(s, info.position));
        }
        unreachable!()
    }
}
