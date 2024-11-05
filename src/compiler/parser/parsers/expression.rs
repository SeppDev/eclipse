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
            Token::Identifier(String::new()),
        ],
        false,
    ) {
        Some(info) => info,
        None => {
            if required {
                let info = tokens.advance();
                tokens.throw_error(format!("Expected expression, got '{}'", info.token), "");
            }
            return None;
        }
    };
    tokens.start();

    let expression = match info.token {
        Token::Integer(integer) => Expression::Value(Value::Integer {
            minus: false,
            integer: integer.clone(),
        }),
        Token::Identifier(name) => parse_identifier(tokens, name),
        _ => panic!(),
    };
    let expression = tokens.create_expression(expression);

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
        None => return Some(expression),
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
        Box::new(expression),
        operator,
        Box::new(second_expression),
    )))
}

fn parse_identifier(tokens: &mut Tokens, name: String) -> Expression {
    let path = if tokens.peek_expect_tokens(vec![Token::DoubleColon], false).is_some() {
        parse_path(tokens, &name)
    } else {
        Path::from(&name)
    };
    
    let info = match tokens.peek_expect_tokens(vec![Token::OpenParen], true) {
        Some(info) => info,
        None => return Expression::GetVariable(path)
    };
    
    match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens);
            Expression::Call(path, arguments)
        }
        _ => panic!(),
    }
}
