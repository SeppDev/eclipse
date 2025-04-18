use crate::{
    compiler::{lexer::token::TokenKind, nodes::ast::RawNode, parser::Parser},
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_variable_decl(&mut self) -> DiagnosticResult<RawNode> {
        let mutable = self.next_if_eq(TokenKind::Mutable)?;
        let name = self.expect_identifier()?.into();
        let data_type = if self.next_if_eq(TokenKind::Colon)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_single(TokenKind::Equals)?;

        let node = Box::new(self.expect_node()?);
        Ok(RawNode::Declare {
            mutable,
            name,
            data_type,
            node,
        })
    }
}
