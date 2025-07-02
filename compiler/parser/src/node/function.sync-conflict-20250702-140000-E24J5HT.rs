use diagnostics::DiagnosticResult;
use shared::position::LocatedAt;
use syntax::{ast::*, token::TokenKind};

use super::super::Parser;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn parse_function(&mut self) -> DiagnosticResult<RawNode> {
        let name = self.expect_identifier()?.into();
        let parameters = self.expect_parameters()?;
        let return_type = self.expect_type()?;
        let node = Box::new(self.expect_node()?);

        let raw = RawNode::Function {
            name,
            parameters,
            return_type,
            node,
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

            let reference = match self.next_if_eq(Ampersand)? {
                Some(t) => Some(LocatedAt::new((), t.position)),
                None => None,
            };
            let mutable = match self.next_if_eq(Mutable)? {
                Some(t) => Some(LocatedAt::new((), t.position)),
                None => None,
            };

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
