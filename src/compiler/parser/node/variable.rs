use crate::{
    common::position::LocatedAt,
    compiler::{
        ast::RawNode, diagnostics::DiagnosticResult, lexer::token::TokenKind, parser::Parser,
    },
};

impl Parser {
    pub fn parse_variable_decl(&mut self) -> DiagnosticResult<RawNode> {
        let mutable = self
            .next_if_eq(TokenKind::Mutable)?
            .and_then(|i| Some(LocatedAt::new((), i.position)));

        let name = self.expect_identifier()?.into();
        let data_type = if self.next_if_eq(TokenKind::Colon)?.is_some() {
            Some(self.expect_type()?)
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
