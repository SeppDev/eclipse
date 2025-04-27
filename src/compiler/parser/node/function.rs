use crate::compiler::{
    diagnostics::DiagnosticResult,
    lexer::token::TokenKind,
    nodes::ast::{Parameter, RawNode, RawParameter},
    parser::Parser,
};

use TokenKind::*;

impl Parser {
    pub fn parse_function(&mut self) -> DiagnosticResult<RawNode> {
        let name = self.expect_identifier()?.into();
        let parameters = self.expect_parameters()?;
        let return_type = self.expect_type()?;
        let body = Box::new(self.expect_node()?);

        let raw = RawNode::Function {
            name,
            parameters,
            return_type,
            body,
        };

        return Ok(raw);
    }
    pub fn expect_parameters(&mut self) -> DiagnosticResult<Vec<Parameter>> {
        self.expect_single(OpenParen)?;
        let mut params = Vec::new();

        loop {
            if self.next_if_eq(CloseParen)?.is_some() {
                break;
            }
            let start = self.start();

            let reference = self.next_if_eq(Ampersand)?;
            let mutable = self.next_if_eq(Mutable)?;
            let name = self.expect_identifier()?.into();
            let data_type = self.expect_type()?;

            let parameter = RawParameter {
                reference,
                mutable,
                name,
                data_type,
            };

            params.push(self.located(parameter, start))
        }

        return Ok(params);
    }
}
