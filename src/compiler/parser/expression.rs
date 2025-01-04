use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{
        ArithmeticOperator, CompareOperator, Expression, Identifier, RawExpression,
    },
};

impl Tokens {
    pub fn parse_expression(&mut self, required: bool) -> CompileResult<Option<Expression>> {
        let info = match self.peek_expect_tokens(
            vec![
                Token::String(String::new()),
                Token::Integer(String::new()),
                Token::Float(String::new()),
                Token::Boolean(true),
                Token::Identifier(String::new()),
                Token::ExclamationMark,
                Token::OpenBracket,
                Token::Asterisk,
                Token::Ampersand,
                Token::Minus,
            ],
            false,
        ) {
            Some(info) => info,
            None => {
                if required {
                    let info = self.peek().clone();
                    self.error(
                        info.location,
                        format!("Expected expression, got '{}'", info.token),
                    );
                }
                return Ok(None);
            }
        };
        self.start()?;

        let base_expression = match info.token {
            Token::Integer(integer) => RawExpression::Integer(integer),
            Token::Float(float) => RawExpression::Float(float),
            Token::Boolean(boolean) => RawExpression::Boolean(boolean),
            Token::Ampersand => {
                let new_expression = self.parse_expression(true)?.unwrap();
                RawExpression::Reference(Box::new(new_expression))
            }
            Token::Asterisk => {
                let new_expression = self.parse_expression(true)?.unwrap();
                RawExpression::DeReference(Box::new(new_expression))
            }
            Token::Minus => {
                let new_expression = self.parse_expression(true)?.unwrap();
                RawExpression::Minus(Box::new(new_expression))
            }
            Token::ExclamationMark => {
                let new_expression = self.parse_expression(true)?.unwrap();
                RawExpression::Not(Box::new(new_expression))
            }
            Token::OpenParen | Token::OpenBracket => {
                let is_tuple = matches!(info.token, Token::OpenParen);
                let mut expressions = Vec::new();
                loop {
                    let new_expression = match self.parse_expression(false)? {
                        Some(expression) => expression,
                        None => {
                            if is_tuple {
                                self.expect_tokens(vec![Token::CloseParen], false)?;
                            } else {
                                self.expect_tokens(vec![Token::CloseBracket], false)?;
                            }
                            break;
                        }
                    };
                    expressions.push(new_expression);
                    let result = if is_tuple {
                        self.expect_tokens(vec![Token::CloseParen, Token::Comma], false)?
                    } else {
                        self.expect_tokens(vec![Token::CloseBracket, Token::Comma], false)?
                    };

                    match result.token {
                        Token::CloseParen | Token::CloseBracket => break,
                        Token::Comma => continue,
                        _ => panic!(),
                    };
                }
                if is_tuple {
                    RawExpression::Tuple(expressions)
                } else {
                    RawExpression::Array(expressions)
                }
            }
            Token::Identifier(name) => {
                let path = self.parse_path_current(name)?;
                
                match self.peek().token {
                    Token::StartScope => {
                        self.advance()?;
                        RawExpression::Struct(path, self.parse_struct_expression_fields()?)
                    }
                    _ => RawExpression::GetPath(path),
                }
            }
            _ => panic!(),
        };

        let mut first = self.create_located(base_expression);
        loop {
            let info = match self.peek_expect_tokens(
                vec![Token::Dot, Token::OpenParen, Token::OpenBracket],
                false,
            ) {
                Some(_) => self.start()?,
                None => break,
            };
            match info.token {
                Token::Dot => {
                    let identifier = self.parse_identifier()?;
                    first = self.create_located(RawExpression::Field(Box::new(first), identifier))
                }
                Token::OpenParen => {
                    let arguments = self.parse_arguments()?;
                    first = self.create_located(RawExpression::Call(Box::new(first), arguments))
                }
                Token::OpenBracket => {
                    let index = self.parse_expression(true)?.unwrap();
                    self.expect_tokens(vec![Token::CloseBracket], false)?;
                    first =
                        self.create_located(RawExpression::Index(Box::new(first), Box::new(index)));
                }
                _ => todo!(),
            }
        }

        let info = match self.peek_expect_tokens(
            vec![
                Token::Plus,
                Token::Minus,
                Token::Asterisk,
                Token::ForwardSlash,
                Token::Percent,
                Token::Compare,
                Token::NotEquals,
                Token::LessThan,
                Token::LessThanOrEquals,
                Token::GreaterThan,
                Token::GreaterThanOrEquals,
            ],
            false,
        ) {
            Some(_) => self.start()?,
            None => return Ok(Some(first)),
        };

        let second_expression = self.parse_expression(true)?.unwrap();
        let mut first_location = first.location.clone();
        first_location.columns.end = second_expression.location.columns.end;

        let is_arithmetic = matches!(
            info.token,
            Token::Plus | Token::Minus | Token::ForwardSlash | Token::Asterisk | Token::Percent
        );

        let mut info = if is_arithmetic {
            let arithmetic_operator = match info.token {
                Token::Plus => ArithmeticOperator::Plus,
                Token::Minus => ArithmeticOperator::Subtract,
                Token::ForwardSlash => ArithmeticOperator::Division,
                Token::Asterisk => ArithmeticOperator::Multiply,
                Token::Percent => ArithmeticOperator::Modulus,
                _ => panic!(),
            };

            self.create_located(RawExpression::BinaryOperation(
                Box::new(first),
                arithmetic_operator,
                Box::new(second_expression),
            ))
        } else {
            let compare_operator = match info.token {
                Token::Compare => CompareOperator::Equals,
                Token::NotEquals => CompareOperator::NotEquals,
                Token::LessThan => CompareOperator::LessThan,
                Token::LessThanOrEquals => CompareOperator::LessThanOrEquals,
                Token::GreaterThan => CompareOperator::GreaterThan,
                Token::GreaterThanOrEquals => CompareOperator::GreaterThanOrEquals,
                _ => panic!(),
            };

            self.create_located(RawExpression::CompareOperation(
                Box::new(first),
                compare_operator,
                Box::new(second_expression),
            ))
        };

        info.location = first_location;
        return Ok(Some(info));
    }
    fn parse_struct_expression_fields(&mut self) -> CompileResult<Vec<(Identifier, Expression)>> {
        let mut fields = Vec::new();
        while self
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_none()
        {
            let name = self.parse_identifier()?;
            self.expect_tokens(vec![Token::Colon], false)?;
            let expression = self.parse_expression(true)?.unwrap();
            self.peek_expect_tokens(vec![Token::Comma], true);
            fields.push((name, expression))
        }
        return Ok(fields);
    }
}
