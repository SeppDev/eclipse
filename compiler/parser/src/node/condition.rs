use diagnostics::DiagnosticResult;
use lexer::token::TokenKind;
use syntax::ast::RawNode;

use crate::Parser;

impl Parser {
    pub fn parse_condition(&mut self) -> DiagnosticResult<RawNode> {
        use TokenKind::*;

        let condition = self.expect_expression()?.into();
        let body = self.expect_expression()?.into();

        let mut conditions = Vec::new();
        let mut else_condition = None;

        loop {
            let info = self.peek();
            match info.kind {
                ElseIf => {
                    self.next()?;
                    let condition = self.expect_expression()?.into();
                    let body = self.expect_expression()?.into();

                    conditions.push((condition, body))
                }
                Else => {
                    self.next()?;
                    else_condition = Some(self.expect_node()?.into());
                    break;
                }
                _ => break,
            };
        }

        Ok(RawNode::Conditional {
            condition,
            body,
            conditions,
            else_condition,
        })
    }
}
