use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::TokenKind,
        nodes::ast::{Node, RawNode},
    },
    diagnostics::DiagnosticResult,
};

mod block;
mod expression;
mod function;
mod keyword;
mod set;
mod variable;

use super::Parser;

impl Parser {
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        use TokenKind::*;
        self.start();

        let info = self.expect(&vec![
            Function, OpenBlock, Return, Break, Continue, Var, Identifier, Integer, Float, Boolean,
            String,
        ])?;

        let raw: RawNode = match info.kind {
            Function => self.parse_function()?,
            OpenBlock => self.parse_block()?,
            Return => self.parse_return()?,
            Break => self.parse_break()?,
            Continue => self.parse_continue()?,
            Var => self.parse_variable_decl()?,
            Identifier if self.peek().kind.is_equals_operator() => {
                self.parse_after_identifier(info.string)?
            }
            Identifier | Integer | Float | Boolean | String => self.parse_expression(info)?,
            _ => unreachable!("{info}"),
        };

        let mut node = self.located(raw);

        if self.next_if_eq(TokenKind::OpenParen)?.is_some() {
            let arguments = self.expect_arguments(TokenKind::CloseParen)?;
            let position = PositionRange::new(node.position.start, self.last_position.end);
            let raw = RawNode::Call(Box::new(node), arguments);
            node = Located::new(raw, position)
        } else if self.next_if_eq(TokenKind::Dot)?.is_some() {
            let new = self.expect_node()?;
            let position = PositionRange::new(node.position.start, self.last_position.end);
            let raw = RawNode::Field(Box::new(node), Box::new(new));
            node = Located::new(raw, position)
        }

        Ok(node)
    }
    pub fn expect_potential_node(&mut self) -> DiagnosticResult<Option<Node>> {
        match self.peek_found(&vec![
            TokenKind::String,
            TokenKind::Integer,
            TokenKind::Boolean,
            TokenKind::Float,
            TokenKind::Identifier,
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
