use common::{layout::ast::RawNode, lexer::token::TokenKind};
use diagnostics::DiagnosticResult;

use crate::Parser;


impl Parser {
    pub fn parse_condition(&mut self) -> DiagnosticResult<RawNode> {
        let condition = self.expect_expression()?.into();
        let body = self.expect_expression()?.into();

        let mut conditions = Vec::new();
        let mut else_condition = None;

        use TokenKind::*;

        loop {
            let info = self.peek();
            match info.kind {
                ElseIf => {
                    let condition = self.expect_expression()?.into();
                    let body = self.expect_expression()?.into();

                    conditions.push((condition, body))
                }
                Else => {}
                _ => break,
            };

            let condition = self.expect_expression()?.into();
            let body = self.expect_expression()?.into();
            else_condition = Some((condition, body));

            break;
        }

        Ok(RawNode::Conditional {
            condition,
            body,
            conditions,
            else_condition,
        })
    }
}
