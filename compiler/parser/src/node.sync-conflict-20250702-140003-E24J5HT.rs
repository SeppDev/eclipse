use diagnostics::DiagnosticResult;
use shared::path::Path;

use super::Parser;
use syntax::ast::{self, UsePath};
use syntax::token::TokenKind::*;

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

impl<'a> Parser<'a> {
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
        self.expect_expression()
    }
}

#[allow(unused)]
fn extract_paths(base: &Path, paths: &UsePath) -> Vec<Path> {
    todo!()
}
