use crate::{
    compiler::{
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_after_identifier(&mut self, name: String) -> DiagnosticResult<RawNode> {
        todo!()
    }
}
