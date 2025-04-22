use crate::{
    compiler::{
        lexer::token::TokenKind,
        nodes::ast::{Node, RawNode},
    },
    diagnostics::DiagnosticResult,
};

mod block;
mod call;
mod expression;
mod function;
mod keyword;
mod set;
mod order_operations;
mod variable;

use super::Parser;

impl Parser {
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        use TokenKind::*;

        if self.peek().kind == Identifier && self.peek_second().kind.is_equals_operation() {
            self.start();
            let info = self.next()?;
            let raw = self.parse_set_operation(info)?;
            return Ok(self.located(raw));
        }

        if self.peek().kind.is_expression_start() {
            return self.expect_expression();
        }

        self.start();
        let info = self.expect(&vec![
            Function, OpenBlock, Return, Break, Continue, Var, Integer, Minus, Float, Boolean,
            String,
        ])?;

        let raw: RawNode = match info.kind {
            Function => self.parse_function()?,
            OpenBlock => self.parse_block()?,
            Var => self.parse_variable_decl()?,

            Return => self.parse_return()?,
            Break => self.parse_break()?,
            Continue => self.parse_continue()?,
            _ => unreachable!(),
        };

        Ok(self.located(raw))
    }
}
