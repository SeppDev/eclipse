use crate::{
    compiler::{
        lexer::token::{TokenInfo, TokenKind},
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

impl Parser {
    pub fn parse_call(&mut self, info: TokenInfo) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Call(info.string, self.expect_arguments(TokenKind::CloseParen)?))
    }
}
