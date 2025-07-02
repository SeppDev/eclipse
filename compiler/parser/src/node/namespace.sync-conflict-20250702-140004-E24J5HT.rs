use diagnostics::DiagnosticResult;
use shared::path::Path;
use syntax::{
    ast::{RawNode, UsePath},
    token::TokenKind,
};

use super::super::Parser;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn parse_use(&mut self) -> DiagnosticResult<RawNode> {
        let path = self.expect_path()?;
        Ok(RawNode::Use(path))
    }
    fn expect_path(&mut self) -> DiagnosticResult<UsePath> {
        let info = self.expect(&vec![Identifier, CloseCurlyBracket])?;
        Ok(match info.kind {
            Identifier if self.peek().kind == DoubleColon => {
                self.next()?;
                let extended = self.expect_path()?;
                UsePath::Extend(info.into(), Box::new(extended))
            }
            Identifier => UsePath::Ident(info.into()),
            OpenCurlyBracket => {
                let mut list = Vec::new();
                while self.next_if_eq(CloseCurlyBracket)?.is_none() {
                    let path = self.expect_path()?;
                    list.push(path);
                }
                UsePath::List(list)
            }
            _ => unreachable!(),
        })
    }
}

#[allow(unused)]
fn extract_paths(base: &Path, paths: &UsePath) -> Vec<Path> {
    todo!()
}
