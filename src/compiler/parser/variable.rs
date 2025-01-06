use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Located, LocatedPath},
    parser::{Node, RawNode},
};

impl Tokens {
    pub fn parse_variable(&mut self) -> CompileResult<Node> {
        let mutable: Option<Located<bool>> = if self
            .peek_expect_tokens(vec![Token::Mutable], false)
            .is_some()
        {
            self.start()?;
            Some(self.create_located(true))
        } else {
            None
        };

        let name = self.parse_identifier()?;

        let data_type = if self.peek_expect_tokens(vec![Token::Colon], true).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        let expression = if self.peek_expect_tokens(vec![Token::Equals], true).is_some() {
            self.parse_expression(false)?
        } else {
            None
        };

        return Ok(self.create_located(RawNode::DeclareVariable {
            name,
            mutable,
            data_type,
            expression,
        }));
    }

    pub fn parse_set_variable(&mut self, path: LocatedPath) -> CompileResult<Node> {
        let expression = self.parse_expression(true)?;
        return Ok(self.create_located(RawNode::SetVariable { path, expression }));
    }
}
