use diagnostics::DiagnosticResult;
use lexer::token::TokenKind::*;
use syntax::ast;

use crate::Parser;

mod attribute;
mod block;
mod condition;
mod expression;
mod function;
mod keyword;
mod modifiers;
mod namespace;
mod semicolon;
mod set;
mod types;
mod variable;

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
        let result = self.expect_expression();
        self.skip_semicolons()?;
        result
    }
}
