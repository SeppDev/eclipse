use common::position::LocatedAt;
use diagnostics::DiagnosticResult;
use lexer::token::TokenKind::*;
use syntax::ast::{self, Parameter, RawNode, RawParameter};

use crate::Parser;

impl Parser {
    pub fn parse_function(&mut self) -> DiagnosticResult<RawNode> {
        let name = self.expect_identifier()?.into();
        let parameters = self.expect_parameters()?;

        let return_type = if self.peek().kind.is_expression_start() {
            self.expect_type()?
        } else {
            let position = self.last_position.end.to_range();

            LocatedAt::new(ast::RawType::Void, position)
        };

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

            let reference: Option<LocatedAt> = match self.next_if_eq(Ampersand)? {
                Some(a) => Some(LocatedAt::new((), a.position)),
                None => None,
            };
            let mutable: Option<LocatedAt> = match self.next_if_eq(Mutable)? {
                Some(a) => Some(LocatedAt::new((), a.position)),
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
