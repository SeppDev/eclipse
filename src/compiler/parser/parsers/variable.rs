use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

impl Tokens {
    pub fn parse_variable(&mut self) -> CompileResult<NodeInfo> {
        let mutable = self
            .peek_expect_tokens(vec![Token::Mutable], true)
            .is_some();
        let name = self.parse_identifier()?;

        let data_type = if self
            .peek_expect_tokens(vec![Token::Colon], true)
            .is_some()
        {
            Some(self.parse_type()?)
        } else {
            None
        };

        let expression = if self
            .peek_expect_tokens(vec![Token::Equals], true)
            .is_some()
        {
            self.parse_expression(false)?
        } else {
            None
        };

        return Ok(self.create_node(Node::DeclareVariable {
            name,
            mutable,
            data_type,
            expression,
        }));
    }

    pub fn parse_set_variable(&mut self, name: String) -> CompileResult<NodeInfo> {
        let expression = self.parse_expression(true)?;
        return Ok(self.create_node(Node::SetVariable { name, expression }));
    }
}
