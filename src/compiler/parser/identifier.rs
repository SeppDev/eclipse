use crate::compiler::{
    errors::CompileResult,
    nodes::ast::{
        self, Expression, Identifier, Located, LocatedPath, Node, RawExpression, RawNode,
    },
};

use super::super::lexer::{Token, Tokens};

impl Tokens {
    pub fn parse_identifier(&mut self) -> CompileResult<Identifier> {
        let info = self.start()?;

        if let Token::Identifier(name) = info.token {
            return Ok(self.create_located(name));
        } else {
            self.error(
                info.position,
                format!("Expected identifier, found '{}'", info.token),
            );
        }
        return Ok(Located::default());
    }
    pub fn parse_after_path(&mut self, path: LocatedPath) -> CompileResult<Node> {
        let info = self.expect_tokens(
            vec![
                Token::OpenParen,
                Token::Equals,
                Token::PlusEquals,
                Token::SubtractEquals,
                Token::DivideEquals,
                Token::MultiplyEquals,
                Token::PercentEquals,
                Token::DoubleColon,
                Token::Increment,
                Token::Decrement,
            ],
            false,
        )?;

        return Ok(match info.token {
            Token::OpenParen => {
                let arguments = self.parse_arguments()?;
                self.create_located(RawNode::Call(path, arguments))
            }
            Token::Increment | Token::Decrement => {
                let variable = Located::new(path.position, RawExpression::GetPath(path.clone()));

                let operator = match info.token {
                    Token::Increment => ast::ArithmeticOperator::Add,
                    Token::Decrement => ast::ArithmeticOperator::Subtract,
                    _ => panic!(),
                };

                let mut position = info.position;
                position.set_end(info.position.end);

                let second_expression =
                    Located::new(position, RawExpression::Integer("1".to_string()));

                let binary_expression = RawExpression::ArithmeticOperation(
                    Box::new(variable),
                    operator,
                    Box::new(second_expression),
                );

                let binary_expression = Located::new(position, binary_expression);

                return Ok(self.create_located(RawNode::SetPath(path, binary_expression)));
            }
            Token::Equals => self.parse_set_variable(path)?,
            Token::PlusEquals
            | Token::SubtractEquals
            | Token::MultiplyEquals
            | Token::DivideEquals
            | Token::PercentEquals => {
                let variable = Located::new(info.position, RawExpression::GetPath(path.clone()));
                let expression = self.parse_expression(true)?.unwrap();

                let operator = match info.token {
                    Token::PlusEquals => ast::ArithmeticOperator::Add,
                    Token::SubtractEquals => ast::ArithmeticOperator::Subtract,
                    Token::MultiplyEquals => ast::ArithmeticOperator::Multiply,
                    Token::DivideEquals => ast::ArithmeticOperator::Divide,
                    Token::PercentEquals => ast::ArithmeticOperator::Remainder,
                    _ => panic!(),
                };

                let binary_expression = RawExpression::ArithmeticOperation(
                    Box::new(variable),
                    operator,
                    Box::new(expression),
                );

                let binary_expression = Located::new(info.position, binary_expression);

                return Ok(self.create_located(RawNode::SetPath(path, binary_expression)));
            }
            _ => {
                let message = format!("Not yet implemented: {info:#?} {}", path.raw);
                self.error(info.position, message);
                return Err(());
            }
        });
    }
}
