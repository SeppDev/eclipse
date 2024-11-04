use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Expression, ExpressionInfo, Operator, Value},
};

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

    let expression = tokens.create_expression(match info.token {
        Token::Integer(integer) => Expression::Value(Value::Integer {
            minus: false,
            integer: integer.clone(),
        }),
        Token::Identifier(name) => Expression::GetVariable(name),
        _ => panic!(),
    });

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
