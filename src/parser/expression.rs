use crate::{
    lexer::{Token, TokensGroup},
    parser::{parser::peek_expect_tokens, Operator},
    CompileError, ParseResult,
};

use super::{arguments::parse_arguments, node::Expression, path::parse_path, Path, Value};

pub fn parse_expression(tokens: &mut TokensGroup) -> ParseResult<Option<Expression>> {
    let info = tokens.peek()?;
    match info.token {
        Token::SemiColon => return Ok(None),
        _ => {}
    };

    let info = tokens.advance()?;
    let expression: Option<Expression> = match info.token {
        Token::String(string) => Some(Expression::Value(Value::String(string))),
        Token::Integer(value) => {
            let integer = match value.parse::<usize>() {
                Ok(int) => int,
                Err(_) => panic!(),
            };
            Some(Expression::Value(Value::Integer(false, integer)))
        }
        Token::Identifier(name) => {
            let extend = peek_expect_tokens(tokens, vec![Token::DoubleColon], true)?;
            let path = if extend.is_none() {
                Path::new(name)
            } else {
                parse_path(tokens, name)?
            };

            let info = peek_expect_tokens(tokens, vec![Token::OpenParen], true)?;
            if info.is_none() {
                Some(Expression::GetVariable(path))
            } else {
                let arguments = parse_arguments(tokens)?;
                Some(Expression::Call(path, arguments))
            }
        }
        _ => None,
    };

    let info = match peek_expect_tokens(
        tokens,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
        ],
        true,
    )? {
        Some(info) => info,
        None => return Ok(expression),
    };

    let operator = match info.token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::Asterisk => Operator::Multiply,
        Token::ForwardSlash => Operator::Division,
        token => todo!("{:?}", token),
    };

    let other = parse_expected_expression(tokens)?;
    let expression =
        Expression::BinaryOperation(Box::new(expression.unwrap()), operator, Box::new(other));

    return Ok(Some(expression));
}

pub fn parse_expected_expression(tokens: &mut TokensGroup) -> ParseResult<Expression> {
    return match parse_expression(tokens)? {
        Some(expr) => Ok(expr),
        None => Err(CompileError::new(
            format!("Expected expression"),
            tokens.current.line,
        )),
    };
}
