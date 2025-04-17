use crate::{
    compiler::{lexer::token::Token, nodes::ast::RawNode, parser::Parser},
    diagnostics::DiagnosticResult,
};

impl Parser {
    pub fn parse_variable_decl(&mut self) -> DiagnosticResult<RawNode> {
        let mutable = self.next_if_eq(Token::Mutable)?;
        let name = self.expect_identifier()?;
        let data_type = if self.next_if_eq(Token::Colon)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_single(Token::Equals)?;

        let node = Box::new(self.expect_node()?);
        Ok(RawNode::Declare {
            mutable,
            name,
            data_type,
            node,
        })
    }
}
