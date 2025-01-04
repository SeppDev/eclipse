use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Node, Parameter, RawNode, RawParameter},
};

impl Tokens {
    pub fn parse_function(&mut self, is_main: bool, key: String) -> CompileResult<Node> {
        let name = self.parse_identifier()?;
        self.expect_tokens(vec![Token::OpenParen], false)?;

        let mut parameters: Vec<Parameter> = Vec::new();
        if self
            .peek_expect_tokens(vec![Token::CloseParen], true)
            .is_none()
        {
            loop {
                self.start_next();

                let mutable = self
                    .peek_expect_tokens(vec![Token::Mutable], true)
                    .is_some();

                let name = self.parse_identifier()?;
                let data_type = self.parse_type()?;

                let mut location = self.pop_start().location;
                let current = self.current();
                location.lines.end = current.location.lines.end;
                location.columns.end = current.location.columns.end;

                let parameter = Parameter {
                    location,
                    raw: RawParameter {
                        mutable,
                        name,
                        data_type,
                    },
                };

                parameters.push(parameter);

                let result = &self.expect_tokens(vec![Token::CloseParen, Token::Comma], false)?;
                match result.token {
                    Token::CloseParen => break,
                    Token::Comma => continue,
                    _ => break,
                }
            }
        }

        let return_type = if self.peek_expect_tokens(vec![Token::Colon], true).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_tokens(vec![Token::StartScope], false)?;
        let body = self.parse_body()?;

        return Ok(self.create_located(RawNode::Function {
            key: if is_main && name.raw == "main" {
                "main".to_string()
            } else {
                key
            },
            name,
            parameters,
            return_type,
            body,
        }));
    }
}
