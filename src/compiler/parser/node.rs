use crate::compiler::{
    diagnostics::DiagnosticResult,
    lexer::token::TokenKind,
    nodes::ast::{Node, RawNode},
};

mod block;
mod expression;
mod function;
mod keyword;
mod namespace;
mod order_operations;
mod set;
mod variable;
mod types;

use super::Parser;
use TokenKind::*;

impl Parser {
    pub fn top_level_expect(&mut self) -> DiagnosticResult<Node> {
        let info = self.peek();
        Ok(match info.kind {
            Import => {
                let start = self.next()?.position.start;
                let name = self.expect_identifier()?;

                self.located(RawNode::Import(name.into()), start)
            }
            Identifier => {
                let start = info.position.start;
                let raw = self.parse_function()?;
                self.located(raw, start)
            }
            _ => return self.expect_node(),
        })
    }
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        if self.peek().kind == Identifier && self.peek_second().kind.is_equals_operation() {
            let start = self.start();
            let info = self.next()?;
            let raw = self.parse_set_operation(info)?;
            return Ok(self.located(raw, start));
        }

        if self.peek().kind.is_expression_start() {
            return self.expect_expression();
        }

        let start = self.start();
        let info = self.expect(&vec![
            OpenBlock, Return, Break, Continue, Var, Integer, Minus, Float, Boolean, String, Use,
        ])?;

        let raw: RawNode = match info.kind {
            OpenBlock => self.parse_block()?,
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
