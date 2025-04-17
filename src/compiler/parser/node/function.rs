use crate::{
    compiler::{
        lexer::token::Token,
        nodes::ast::{Parameter, RawNode, RawParameter},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_function(&mut self) -> DiagnosticResult<RawNode> {
        let name = self.expect_identifier()?;
        let parameters = self.expect_parameters()?;
        let return_type = if self.next_if_eq(Token::Colon)?.is_some() {
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
        self.expect_single(Token::OpenParen)?;
        let mut params = Vec::new();

        loop {
            if self.next_if_eq(Token::CloseParen)?.is_some() {
                break;
            }
            self.start();

            let reference = self.next_if_eq(Token::Ampersand)?;
            let mutable = self.next_if_eq(Token::Mutable)?;
            let name = self.expect_identifier()?;
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
