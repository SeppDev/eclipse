use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Expression, ExpressionInfo, Operator, Value},
    path::Path, types::ReferenceManager,
};

use super::{arguments::parse_arguments, path::parse_path};

pub fn parse_expression(
    tokens: &mut Tokens,
    required: bool,
) -> CompileResult<Option<ExpressionInfo>> {
    let info = match tokens.peek_expect_tokens(
        vec![
            Token::String(String::new()),
            Token::Integer(String::new()),
            Token::Float(String::new()),
            Token::Boolean(true),
            Token::Identifier(String::new()),
            Token::Asterisk,
            Token::Ampersand,
            Token::Minus,
            Token::OpenParen,
        ],
        false,
    ) {
        Some(info) => info,
        None => {
            if required {
                let info = tokens.advance();
                tokens.error(
                    info.location,
                    format!("Expected expression, got '{}'", info.token),
                );
            }
            return Ok(None);
        }
    };
    tokens.start();

    let expression = match info.token {
        Token::Integer(integer) => Expression::Value(Value::Integer(integer)),
        Token::Float(float) => Expression::Value(Value::Float(float)),
        Token::String(string) => Expression::Value(Value::StaticString(string)),
        Token::Boolean(boolean) => Expression::Value(Value::Boolean(boolean)),
        Token::Ampersand => {
            let mut new_expression = parse_expression(tokens, true)?.unwrap();
            let info = tokens.pop_start();
            new_expression.add_reference().unwrap();
            new_expression.location.columns.start = info.location.columns.start;
            
            return Ok(Some(new_expression))
        }
        Token::Asterisk => {
            let mut new_expression = parse_expression(tokens, true)?.unwrap();
            let info = tokens.pop_start();
            new_expression.add_pointer().unwrap();
            new_expression.location.columns.start = info.location.columns.start;
            
            return Ok(Some(new_expression))
        }
        Token::Minus => {
            let new_expression = parse_expression(tokens, true)?.unwrap();
            return Ok(Some(
                tokens.create_expression(Expression::Minus(Box::new(new_expression))),
            ));
        }
        Token::ExclamationMark => {
            let new_expression = parse_expression(tokens, true)?.unwrap();
            return Ok(Some(
                tokens.create_expression(Expression::Not(Box::new(new_expression))),
            ));
        }
        Token::OpenParen => {
            let mut expressions = Vec::new();
            loop {
                let new_expression = match parse_expression(tokens, false)? {
                    Some(expression) => expression,
                    None => {
                        tokens.expect_tokens(vec![Token::CloseParen], false);
                        break;
                    }
                };
                expressions.push(new_expression);
                let result = tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false);
                match result.token {
                    Token::CloseParen => break,
                    Token::Comma => continue,
                    _ => panic!(),
                };
            }
            return Ok(Some(
                tokens.create_expression(Expression::Tuple(expressions)),
            ));
        }
        Token::Identifier(name) => parse_identifier(tokens, name)?,
        _ => panic!(),
    };
    let first_expression_info = tokens.create_expression(expression);

    let info = match tokens.peek_expect_tokens(
        vec![
            Token::Plus,
            Token::Minus,
            Token::ForwardSlash,
            Token::Asterisk,
            Token::Compare,
            Token::NotEquals,
            Token::LessThan,
            Token::LessThanOrEquals,
            Token::GreaterThan,
            Token::GreaterThanOrEquals,
        ],
        false,
    ) {
        Some(_) => tokens.start(),
        None => return Ok(Some(first_expression_info)),
    };
    let operator = match info.token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::ForwardSlash => Operator::Division,
        Token::Asterisk => Operator::Multiply,

        Token::Compare => Operator::Equals,
        Token::NotEquals => Operator::NotEquals,
        Token::LessThan => Operator::LessThan,
        Token::LessThanOrEquals => Operator::LessThanOrEquals,
        Token::GreaterThan => Operator::GreaterThan,
        Token::GreaterThanOrEquals => Operator::GreaterThanOrEquals,
        _ => panic!(),
    };

    let second_expression = parse_expression(tokens, true)?.unwrap();
    let mut first_location = first_expression_info.location.clone();
    first_location.columns.end = second_expression.location.columns.end;

    let mut info = tokens.create_expression(Expression::BinaryOperation(
        Box::new(first_expression_info),
        operator,
        Box::new(second_expression),
    ));
    info.location = first_location;
    return Ok(Some(info));
}

fn parse_identifier(tokens: &mut Tokens, name: String) -> CompileResult<Expression> {
    let path = if tokens
        .peek_expect_tokens(vec![Token::DoubleColon], false)
        .is_some()
    {
        parse_path(tokens, &name)?
    } else {
        Path::from(&name)
    };

    let info = match tokens.peek_expect_tokens(vec![Token::OpenParen], true) {
        Some(info) => info,
        None => return Ok(Expression::GetVariable(path)),
    };

    match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;
            Ok(Expression::Call(path, arguments))
        }
        _ => panic!(),
    }
}
