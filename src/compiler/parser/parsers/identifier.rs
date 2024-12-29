use crate::compiler::{
    errors::CompileResult,
    parser::{ArithmeticOperator, Expression, ExpressionInfo, Node, NodeInfo},
    path::Path,
};

use super::super::super::lexer::{Token, Tokens};
impl Tokens {
    pub fn parse_identifier(&mut self) -> CompileResult<String> {
        let current = self.current().clone();
        let info = self.advance()?;

        if let Token::Identifier(name) = info.token {
            return Ok(name);
        } else {
            self.error(
                current.location,
                format!("Expected identifier, found '{}'", info.token),
            );
        }
        return Ok("x".to_string());
    }
    pub fn parse_after_identifier(&mut self, name: String) -> CompileResult<NodeInfo> {
        let info = self.peek_require_tokens(vec![
            Token::OpenParen,
            Token::Equals,
            Token::PlusEquals,
            Token::SubtractEquals,
            Token::DivideEquals,
            Token::MultiplyEquals,
            Token::PercentEquals,
            Token::DoubleColon,
        ])?;

        match info.token {
            Token::DoubleColon => {
                let path = self.parse_path(&name)?;
                let _ = self.expect_tokens(vec![Token::OpenParen], false);
                let arguments = self.parse_arguments()?;
                return Ok(self.create_node(Node::Call(path, arguments)));
            }
            _ => {}
        }

        self.advance()?;
        return Ok(match info.token {
            Token::OpenParen => {
                let arguments = self.parse_arguments()?;
                self.create_node(Node::Call(Path::from(&name), arguments))
            }
            Token::Equals => self.parse_set_variable(name.clone())?,
            Token::PlusEquals
            | Token::SubtractEquals
            | Token::MultiplyEquals
            | Token::DivideEquals
            | Token::PercentEquals => {
                let variable = ExpressionInfo {
                    location: info.location.clone(),
                    expression: Expression::GetVariable(Path::from(&name)),
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

                let binary_expression =
                    Expression::BinaryOperation(Box::new(variable), operator, Box::new(expression));

                let binary_expression = ExpressionInfo {
                    location: info.location.clone(),
                    expression: binary_expression,
                };

                return Ok(self.create_node(Node::SetVariable {
                    name,
                    expression: Some(binary_expression),
                }));
            }
            _ => {
                let message = format!("{info:#?} {name}");
                self.error(info.location, message);
                return Err(());
            }
        });
    }
}