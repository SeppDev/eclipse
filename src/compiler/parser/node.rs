use crate::{
    compiler::{
        lexer::token::TokenKind,
        nodes::ast::{Node, RawNode},
    },
    diagnostics::DiagnosticResult,
};

mod block;
mod function;
mod keyword;
mod set;
mod variable;

use super::Parser;

impl Parser {
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        self.start();

        let token = self.next()?;
        let raw: RawNode = match token.kind {
            TokenKind::Function => self.parse_function()?,
            TokenKind::OpenBlock => self.parse_block()?,
            TokenKind::Return => self.parse_return()?,
            TokenKind::Break => self.parse_break()?,
            TokenKind::Continue => self.parse_continue()?,
            TokenKind::Var => self.parse_variable_decl()?,
            TokenKind::Identifier if self.peek().kind.is_equals_operator() => {
                self.parse_after_identifier(token.string)?
            }
            TokenKind::Integer => RawNode::Integer(token.string),
            TokenKind::Float => RawNode::Float(token.string),
            _ => unreachable!("{token}"),
        };

        Ok(self.located(raw))
    }
    pub fn expect_potential_node(&mut self) -> DiagnosticResult<Option<Node>> {
        match self.peek_found(vec![
            TokenKind::String,
            TokenKind::Integer,
            TokenKind::Float,
            TokenKind::Identifier,
            TokenKind::Boolean,
            TokenKind::ExclamationMark,
            TokenKind::OpenBracket,
            TokenKind::OpenParen,
            TokenKind::Asterisk,
            TokenKind::Ampersand,
            TokenKind::Minus,
        ]) {
            Some(_) => Ok(Some(self.expect_node()?)),
            None => Ok(None),
        }
    }
}
