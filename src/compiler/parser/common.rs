use crate::{
    common::position::Located,
    compiler::{
        lexer::{
            kind::LocatedString,
            token::{Token, TokenInfo},
        },
        nodes::parser::ParserState,
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

impl Located<ParserState> {
    pub fn is_block(&self) -> bool {
        match self.raw {
            ParserState::Block { .. }
            | ParserState::OpenBlock { .. }
            | ParserState::Function { .. } => true,
            _ => false,
        }
    }
    pub fn is_node(&self) -> bool {
        match self.raw {
            ParserState::Return(..)
            | ParserState::Continue(..)
            | ParserState::Break(..)
            | ParserState::VarDecl { .. }
            | ParserState::Conditional { .. } => true,
            _ => false,
        }
    }
    pub fn is_expression(&self) -> bool {
        match self.raw {
            ParserState::Identifier(..)
            | ParserState::Integer(..)
            | ParserState::Float(..)
            | ParserState::Block { .. } => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match self.raw {
            ParserState::Operator(..) | ParserState::ArithmeticOperator(..) => true,
            _ => false,
        }
    }
    pub fn insert(&mut self, state: Located<ParserState>) -> Result<(), Located<ParserState>> {
        let body = match &mut self.raw {
            ParserState::VarDecl { value, .. }
            | ParserState::Return(value)
            | ParserState::Continue(value)
            | ParserState::Break(value)
                if node_can_insert(&value, &state) =>
            {
                value
            }
            // ParserState::Conditional { condition, body } => {}
            ParserState::Function { body, .. } | ParserState::OpenBlock { body, .. } => body,
            _ => return Err(state),
        };

        body.push(state);

        Ok(())
    }
}
fn node_can_insert(body: &Vec<Located<ParserState>>, state: &Located<ParserState>) -> bool {
    let last = match body.last() {
        Some(l) => l,
        _ if state.is_expression() | state.is_operator() => return true,
        _ => return false,
    };

    if state.is_operator() {
        return last.is_expression();
    } else if state.is_expression() {
        return last.is_operator();
    }

    return false;
}

// pub fn can_insert(&self, state: &ParserState) -> bool {
//     let expressions = match self {
//         ParserState::Return(value) | ParserState::VarDecl { value, .. } => value,
//         ParserState::Conditional { condition, body } => {
//             if Self::_can_insert(condition, state) {
//                 return true;
//             }
//             return Self::_can_insert(body, state);
//         }
//         _ => return false,
//     };

//     Self::_can_insert(expressions, state)
// }
