use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Function, Located, Parameter, RawFunction, RawParameter, RawType, Type},
};

impl Tokens {
    pub fn parse_function(&mut self, is_main: bool, key: String) -> CompileResult<Function> {
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

                let mut position = self.pop_start().position;
                let current = self.current();

                position.set_end(current.position.end);

                let parameter = Located::new(
                    position,
                    RawParameter {
                        mutable,
                        name,
                        data_type,
                    },
                );

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
            self.parse_type()?
        } else {
            Located::new(name.position, RawType::Void)
        };

        self.expect_tokens(vec![Token::StartScope], false)?;
        let body = self.parse_body()?;

        return Ok(self.create_located(RawFunction {
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
