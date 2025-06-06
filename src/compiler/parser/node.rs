use crate::compiler::{
    diagnostics::DiagnosticResult,
    lexer::token::TokenKind,
};

mod block;
mod error_message;
mod expression;
mod function;
mod keyword;
mod modifiers;
mod namespace;
mod semicolon;
mod set;
mod types;
mod variable;

use super::{ast, Parser};
use TokenKind::*;

impl Parser {
    pub fn top_level_expect(&mut self) -> DiagnosticResult<ast::Node> {
        let peeked = self.peek();

        Ok(match &peeked.kind {
            Import => {
                let start = self.next()?.position.start;
                let name = self.expect_identifier()?;

                self.located(ast::RawNode::Import(name.into()), start)
            }
            _ => return self.expect_node(),
        })
    }
    pub fn expect_node(&mut self) -> DiagnosticResult<ast::Node> {
        self.skip_semicolons()?;

        let peeked = self.peek();
        let start = self.start();

        match &peeked.kind {
            Identifier if self.peek_second().kind.is_equals_operation() => {
                let start = self.start();
                let info = self.next()?;
                let raw = self.parse_set_operation(info)?;
                return Ok(self.located(raw, start));
            }
            kind if kind.is_expression_start() => return self.expect_expression(),
            kind if kind.is_modifier() => {
                let value = self.expect_modifiers_node()?;
                return Ok(self.located(value, start));
            }
            _ => {}
        }

        let info = self.expect(&vec![
            OpenCurlyBracket,
            Return,
            Break,
            Continue,
            Var,
            Integer,
            Minus,
            Float,
            Boolean,
            String,
            Use,
            Function,
        ])?;

        let raw: ast::RawNode = match info.kind {
            Function => self.parse_function()?,
            OpenCurlyBracket => self.parse_block()?,
            Var => self.parse_variable_decl()?,
            Return => self.parse_return()?,
            Break => self.parse_break()?,
            Continue => self.parse_continue()?,
            Use => self.parse_use()?,
            _ => unreachable!(),
        };

        Ok(self.located(raw, start))
    }
}
