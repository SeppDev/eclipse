use crate::{
    compiler::{
        lexer::token::Token,
        nodes::ast::{Node, RawNode},
    },
    diagnostics::DiagnosticResult,
};

mod block;
mod function;
mod keyword;
mod variable;

use super::Parser;

impl Parser {
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        self.start();

        let token = self.next()?;
        let raw: RawNode = match token.raw {
            Token::Function => self.parse_function()?,
            Token::OpenBlock => self.parse_block()?,
            Token::Return => self.parse_return()?,
            Token::Break => self.parse_break()?,
            Token::Continue => self.parse_continue()?,
            Token::VariableDecl => self.parse_variable_decl()?,
            Token::Integer(string) => RawNode::Integer(string),
            Token::Float(string) => RawNode::Float(string),
            Token::Identifier(string) => RawNode::Identifier(string),
            _ => unreachable!("{token:?}"),
        };

        Ok(self.located(raw))
    }
    pub fn expect_potential_node(&mut self) -> DiagnosticResult<Option<Node>> {
        match self.peek_found(vec![
            Token::String(String::new()),
            Token::Integer(String::new()),
            Token::Float(String::new()),
            Token::Identifier(String::new()),
            Token::Boolean(true),
            Token::ExclamationMark,
            Token::OpenBracket,
            Token::OpenParen,
            Token::Asterisk,
            Token::Ampersand,
            Token::Minus,
        ]) {
            Some(_) => Ok(Some(self.expect_node()?)),
            None => Ok(None),
        }
    }
}
