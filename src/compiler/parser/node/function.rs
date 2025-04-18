use crate::{
    compiler::{
        lexer::token::TokenKind, nodes::ast::{Parameter, RawNode, RawParameter}, parser::Parser
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_function(&mut self) -> DiagnosticResult<RawNode> {
        let name = self.expect_identifier()?.into();
        let parameters = self.expect_parameters()?;
        let return_type = if self.next_if_eq(TokenKind::Colon)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

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
        self.expect_single(TokenKind::OpenParen)?;
        let mut params = Vec::new();

        loop {
            if self.next_if_eq(TokenKind::CloseParen)?.is_some() {
                break;
            }
            self.start();

            let reference = self.next_if_eq(TokenKind::Ampersand)?;
            let mutable = self.next_if_eq(TokenKind::Mutable)?;
            let name = self.expect_identifier()?.into();
            let data_type = self.parse_type()?;

            let parameter = RawParameter {
                reference,
                mutable,
                name,
                data_type,
            };

            params.push(self.located(parameter))
        }

        return Ok(params);
    }
}
