use crate::compiler::{
    errors::CompileResult,
    nodes::ast::{
        ArithmeticOperator, Expression, Identifier, Located, LocatedPath, Node, RawExpression,
        RawNode,
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
                info.location,
                format!("Expected identifier, found '{}'", info.token),
            );
        }
        return Ok(Located::default());
    }
    pub fn parse_after_path(&mut self, path: LocatedPath) -> CompileResult<Node> {
        let info = self.expect_tokens(vec![
            Token::OpenParen,
            Token::Equals,
            Token::PlusEquals,
            Token::SubtractEquals,
            Token::DivideEquals,
            Token::MultiplyEquals,
            Token::PercentEquals,
            Token::DoubleColon,
        ], false)?;

        return Ok(match info.token {
            Token::OpenParen => {
                let arguments = self.parse_arguments()?;
                self.create_located(RawNode::Call(path, arguments))
            }
            Token::Equals => self.parse_set_variable(path)?,
            Token::PlusEquals
            | Token::SubtractEquals
            | Token::MultiplyEquals
            | Token::DivideEquals
            | Token::PercentEquals => {
                let variable = Expression {
                    location: info.location.clone(),
                    raw: RawExpression::GetPath(path.clone()),
                };
                let expression = self.parse_expression(true)?.unwrap();

                let operator = match info.token {
                    Token::PlusEquals => ArithmeticOperator::Plus,
                    Token::SubtractEquals => ArithmeticOperator::Subtract,
                    Token::MultiplyEquals => ArithmeticOperator::Multiply,
                    Token::DivideEquals => ArithmeticOperator::Division,
                    Token::PercentEquals => ArithmeticOperator::Modulus,
                    _ => panic!(),
                };

                let binary_expression = RawExpression::BinaryOperation(
                    Box::new(variable),
                    operator,
                    Box::new(expression),
                );

                let binary_expression = Expression {
                    location: info.location.clone(),
                    raw: binary_expression,
                };

                return Ok(self.create_located(RawNode::SetVariable {
                    path,
                    expression: Some(binary_expression),
                }));
            }
            _ => {
                let message = format!("Not yet implemented: {info:#?} {}", path.raw);
                self.error(info.location, message);
                return Err(());
            }
        });
    }
}
