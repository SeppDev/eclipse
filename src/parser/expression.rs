use crate::{
    lexer::{Token, TokensGroup},
    BuildError,
};

use super::{
    arguments::parse_arguments,
    node::{Expression, Value},
    path::parse_path,
    tokens_expected_got, Operator,
};

pub fn parse_expression(tokens: &mut TokensGroup) -> Result<Option<Expression>, BuildError> {
    let info = tokens.peek()?;
    match info.token {
        Token::SemiColon => return Ok(None),
        _ => {}
    };

    let info = tokens.peek()?;
    let minus = match info.token {
        Token::Minus => match tokens.advance() {
            Ok(_) => true,
            Err(error) => return Err(error),
        },
        _ => false,
    };

    let info = tokens.advance()?;
    let expression: Expression = match info.token {
        Token::String(string) => Expression::Value(Value::String(string)),
        Token::Float(string) => {
            let float = match string.parse::<f64>() {
                Ok(f) => f,
                Err(_error) => todo!(),
            };
            let value;

            if minus {
                value = Value::Float(-float);
            } else {
                value = Value::Float(float);
            }
            Expression::Value(value)
        }
        Token::Integer(string) => {
            let integer = match string.parse::<usize>() {
                Ok(int) => int,
                Err(_error) => todo!(),
            };
            let value;

            if minus {
                value = Value::Integer(-(integer as isize));
            } else {
                value = Value::UInteger(integer);
            }
            Expression::Value(value)
        }
        Token::Identifier(name) => {
            let path = match parse_path(tokens, name) {
                Ok(path) => path,
                Err(error) => return Err(error),
            };

            match tokens.peek() {
                Ok(info) => match info.token {
                    Token::OpenParen => {
                        tokens.advance()?;
                        let arguments = parse_arguments(tokens)?;
                        Expression::Call(path, arguments)
                    }
                    // Token::SemiColon => Expression::GetVariable(path),
                    _ => Expression::GetVariable(path),
                },
                Err(error) => return Err(error),
            }
        }
        _ => return Ok(None),
    };

    let info = tokens.peek()?;
    match info.token {
        Token::SemiColon | Token::Comma | Token::CloseParen => return Ok(Some(expression)),
        Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
            tokens.advance()?;

            let operator = match info.token {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                Token::Asterisk => Operator::Multiply,
                Token::Slash => Operator::Division,
                _ => panic!(),
            };

            let second = match parse_expression(tokens) {
                Ok(expression) => match expression {
                    Some(expression) => expression,
                    None => return Err(tokens.create_error(format!("Expression expected"))),
                },
                Err(error) => return Err(error),
            };
            return Ok(Some(Expression::BinaryOperation(
                Box::new(expression),
                operator,
                Box::new(second),
            )));
        }
        _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
    }
}
