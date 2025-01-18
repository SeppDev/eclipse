use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{
        ArithmeticOperator, CompareOperator, Expression, Identifier, Located, RawExpression,
    },
};

impl Tokens {
    pub fn parse_expression(&mut self, required: bool) -> CompileResult<Option<Expression>> {
        let expression = match self.parse_base_expression(required)? {
            Some(e) => e,
            None => return Ok(None),
        };

        return self.parse_expression_after(expression);
    }
    fn parse_base_expression(&mut self, required: bool) -> CompileResult<Option<Expression>> {
        let info = match self.peek_expect_tokens(
            vec![
                Token::String(String::new()),
                Token::Integer(String::new()),
                Token::Float(String::new()),
                Token::Identifier(String::new()),
                Token::Boolean(true),
                Token::ExclamationMark,
                Token::OpenBracket,
                Token::OpenParen,
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
                let new_expression = self.parse_base_expression(true)?.unwrap();
                RawExpression::Reference(Box::new(new_expression))
            }
            Token::Asterisk => {
                let new_expression = self.parse_base_expression(true)?.unwrap();
                RawExpression::DeReference(Box::new(new_expression))
            }
            Token::Minus => {
                let new_expression = self.parse_base_expression(true)?.unwrap();
                RawExpression::Minus(Box::new(new_expression))
            }
            Token::ExclamationMark => {
                let new_expression = self.parse_base_expression(true)?.unwrap();
                RawExpression::Not(Box::new(new_expression))
            }
            Token::OpenBracket => {
                let mut expressions = Vec::new();
                loop {
                    let new_expression = match self.parse_expression(false)? {
                        Some(expression) => expression,
                        None => {
                            self.expect_tokens(vec![Token::CloseBracket], false)?;
                            break;
                        }
                    };
                    expressions.push(new_expression);
                    let result =
                        self.expect_tokens(vec![Token::CloseBracket, Token::Comma], false)?;

                    match result.token {
                        Token::CloseBracket => break,
                        Token::Comma => continue,
                        _ => panic!(),
                    };
                }

                RawExpression::Array(expressions)
            }
            Token::OpenParen => {
                let mut expressions = self.parse_arguments()?;
                if expressions.len() == 1 {
                    RawExpression::Group(Box::new(expressions.pop().unwrap()))
                } else {
                    RawExpression::Tuple(expressions)
                }
            }
            Token::Identifier(name) => {
                let path = self.parse_path_current(name)?;

                match self.peek().token {
                    Token::StartScope => {
                        self.advance()?;
                        RawExpression::InvokeStruct(path, self.parse_struct_expression_fields()?)
                    }
                    _ => RawExpression::GetPath(path),
                }
            }
            _ => panic!(),
        };

        let mut expression = self.create_located(base_expression);
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
                    expression =
                        self.create_located(RawExpression::Field(Box::new(expression), identifier))
                }
                Token::OpenParen => {
                    let arguments = self.parse_arguments()?;
                    expression =
                        self.create_located(RawExpression::Invoke(Box::new(expression), arguments))
                }
                Token::OpenBracket => {
                    let index = self.parse_expression(true)?.unwrap();
                    self.expect_tokens(vec![Token::CloseBracket], false)?;
                    expression = self.create_located(RawExpression::Index(
                        Box::new(expression),
                        Box::new(index),
                    ));
                }
                _ => todo!(),
            }
        }

        let info = self.peek_expect_tokens(
            vec![
                Token::Compare,
                Token::NotEquals,
                Token::LessThan,
                Token::LessThanOrEquals,
                Token::GreaterThan,
                Token::GreaterThanOrEquals,
            ],
            true,
        );

        let info = match info {
            Some(i) => i,
            None => return Ok(Some(expression)),
        };

        let operator = match info.token {
            Token::Compare => CompareOperator::Equals,
            Token::NotEquals => CompareOperator::NotEquals,
            Token::LessThan => CompareOperator::LessThan,
            Token::LessThanOrEquals => CompareOperator::LessThanOrEquals,
            Token::GreaterThan => CompareOperator::GreaterThan,
            Token::GreaterThanOrEquals => CompareOperator::GreaterThanOrEquals,
            _ => panic!(),
        };

        let second = self.parse_expression(true)?.unwrap();

        let mut location = expression.location.clone();
        location.set_end(&second.location);

        let new_raw_expression =
            RawExpression::CompareOperation(Box::new(expression), operator, Box::new(second));

        return Ok(Some(Located::new(location, new_raw_expression)));
    }
    fn parse_expression_after(
        &mut self,
        first_expression: Expression,
    ) -> CompileResult<Option<Expression>> {
        #[derive(Debug)]
        enum Output {
            Operator(ArithmeticOperator),
            Expression(Expression),
        }

        let mut operator_stack: Vec<ArithmeticOperator> = Vec::with_capacity(4);
        let mut output: Vec<Output> = Vec::new();

        output.push(Output::Expression(first_expression));

        loop {
            let info = match self.peek_expect_tokens(
                vec![
                    Token::Plus,
                    Token::Minus,
                    Token::Asterisk,
                    Token::ForwardSlash,
                    Token::Percent,
                    Token::LeftBitshift,
                    Token::RightBitshift,
                ],
                true,
            ) {
                Some(i) => i,
                None => break,
            };

            let operator = match info.token {
                Token::Plus => ArithmeticOperator::Add,
                Token::Minus => ArithmeticOperator::Subtract,
                Token::Asterisk => ArithmeticOperator::Multiply,
                Token::ForwardSlash => ArithmeticOperator::Divide,
                Token::Percent => ArithmeticOperator::Remainder,
                Token::LeftBitshift => ArithmeticOperator::LeftBitshift,
                Token::RightBitshift => ArithmeticOperator::RightBitshift,
                _ => panic!(),
            };

            loop {
                let last = match operator_stack.last() {
                    Some(last) => last,
                    None => {
                        operator_stack.push(operator);
                        break;
                    }
                };
                if last.precedence() >= operator.precedence() {
                    output.push(Output::Operator(operator_stack.pop().unwrap()));
                } else {
                    operator_stack.push(operator);
                    break;
                }
            }

            let expression = self.parse_base_expression(true)?.unwrap();

            output.push(Output::Expression(expression));
        }

        while let Some(operator) = operator_stack.pop() {
            output.push(Output::Operator(operator));
        }

        if output.len() == 1 {
            if let Some(Output::Expression(e)) = output.pop() {
                return Ok(Some(e));
            }
        }

        output.reverse();

        let mut solve_stack: Vec<Expression> = Vec::new();

        loop {
            let operator = match output.pop() {
                Some(o) => match o {
                    Output::Operator(o) => o,
                    Output::Expression(e) => {
                        solve_stack.push(e);
                        continue;
                    }
                },
                None => break Ok(solve_stack.pop()),
            };

            let second = solve_stack.pop().unwrap();
            let first = solve_stack.pop().unwrap();

            let location = first.location.clone();

            let raw =
                RawExpression::ArithmeticOperation(Box::new(first), operator, Box::new(second));
            let located = Located::new(location, raw);

            solve_stack.push(located);
        }
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

impl ArithmeticOperator {
    fn precedence(&self) -> u8 {
        match self {
            Self::Add | Self::Subtract => 1,
            Self::Multiply | Self::Divide | Self::Remainder => 2,
            Self::LeftBitshift | Self::RightBitshift => 3,
        }
    }
}
