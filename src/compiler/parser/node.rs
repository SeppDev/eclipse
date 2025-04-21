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
mod variable;

use super::Parser;

impl Parser {
    pub fn expect_node(&mut self) -> DiagnosticResult<Node> {
        use TokenKind::*;

        if self.peek().kind.is_expression_start() {
            return self.expect_expression();
        }

        self.start();
        let info = self.expect(&vec![
            Function, OpenBlock, Return, Break, Continue, Var, Identifier, Integer, Minus, Float,
            Boolean, String,
        ])?;

        let raw: RawNode = match info.kind {
            Function => self.parse_function()?,
            OpenBlock => self.parse_block()?,
            Var => self.parse_variable_decl()?,
            Identifier if self.peek().kind.is_equals_operation() => {
                self.parse_after_identifier(info)?
            }
            Return => self.parse_return()?,
            Break => self.parse_break()?,
            Continue => self.parse_continue()?,
            // Identifier if self.peek().kind == TokenKind::OpenParen => {
            //     self.next()?;
            //     self.parse_call(info)?
            // }
            // Minus => self.parse_expression(info)?,
            // _ if info.kind.is_expression() => self.parse_expression(info)?,
            _ => unreachable!(),
        };
        let node = self.located(raw);

        Ok(node)
    }
    // pub fn expect_potential_node(&mut self) -> DiagnosticResult<Option<Node>> {
    //     let peek = self.peek();
    //     let kind = &peek.kind;

    //     if kind.is_expression() || kind.is_arithmetic_operator() {
    //         return Ok(Some(self.expect_node()?));
    //     }
    //     return Ok(None);
    // }
}

// let mut node = self.located(raw);
// loop {
//     if self.next_if_eq(TokenKind::Dot)?.is_some() {
//         self.start();
//         let field = self.expect(&vec![Identifier, Integer])?;
//         node = self.located(RawNode::Field(Box::new(node), field.string));
//         continue;
//     }
//     break;
// }
