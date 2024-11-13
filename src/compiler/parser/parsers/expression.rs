use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Expression, ExpressionInfo, Operator, Value},
    path::Path,
};

use super::{arguments::parse_arguments, path::parse_path};

pub fn parse_expression(tokens: &mut Tokens, required: bool) -> Option<ExpressionInfo> {
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
                tokens.throw_error(format!("Expected expression, got '{}'", info.token), "", info.location);
            }
            return None;
        }
    };
    tokens.start();

    let expression = match info.token {
        Token::Integer(integer) => Expression::Value(Value::Integer(integer)),
        Token::Float(float) => Expression::Value(Value::Float(float)),
        Token::String(string) => Expression::Value(Value::StaticString(string)),
        Token::Boolean(boolean) => Expression::Value(Value::Boolean(boolean)),
        Token::Ampersand => {
            let new_expression = parse_expression(tokens, true).unwrap();
            return Some(tokens.create_expression(Expression::Reference(Box::new(new_expression))));
        }
        Token::Asterisk => {
            let new_expression = parse_expression(tokens, true).unwrap();
            return Some(tokens.create_expression(Expression::Pointer(Box::new(new_expression))));
        }
        Token::Minus => {
            let new_expression = parse_expression(tokens, true).unwrap();
            return Some(tokens.create_expression(Expression::Minus(Box::new(new_expression))));
        }
        Token::OpenParen => {
            let mut expressions = Vec::new();
            loop {
                let new_expression = match parse_expression(tokens, false) {
                    Some(expression) => expression,
                    None => {
                        tokens.expect_tokens(vec![Token::CloseParen], false);
                        break;
                    }
                };
                expressions.push(new_expression);
                match tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false).token {
                    Token::CloseParen => break,
                    Token::Comma => continue,
                    _ => panic!()
                };
            }
            return Some(tokens.create_expression(Expression::Tuple(expressions)));
        }
        Token::Identifier(name) => parse_identifier(tokens, name),
        _ => panic!(),
    };
    let expression_info = tokens.create_expression(expression);

    let info = match tokens.peek_expect_tokens(
        vec![
            Token::Plus,
            Token::Minus,
            Token::ForwardSlash,
            Token::Asterisk,
        ],
        false,
    ) {
        Some(_) => tokens.start(),
        None => return Some(expression_info),
    };
    let operator = match info.token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::ForwardSlash => Operator::Division,
        Token::Asterisk => Operator::Multiply,
        _ => panic!(),
    };

    let second_expression = parse_expression(tokens, true).unwrap();

    Some(tokens.create_expression(Expression::BinaryOperation(
        Box::new(expression_info),
        operator,
        Box::new(second_expression),
    )))
}

fn parse_identifier(tokens: &mut Tokens, name: String) -> Expression {
    let path = if tokens
        .peek_expect_tokens(vec![Token::DoubleColon], false)
        .is_some()
    {
        parse_path(tokens, &name)
    } else {
        Path::from(&name)
    };

    let info = match tokens.peek_expect_tokens(vec![Token::OpenParen], true) {
        Some(info) => info,
        None => return Expression::GetVariable(path),
    };

    match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens);
            Expression::Call(path, arguments)
        }
        _ => panic!(),
    }
}
