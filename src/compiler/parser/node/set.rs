use crate::{
    compiler::{lexer::token::TokenInfo, nodes::ast::RawNode, parser::Parser},
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_after_identifier(&mut self, name: TokenInfo) -> DiagnosticResult<RawNode> {
        todo!()
    }
}
