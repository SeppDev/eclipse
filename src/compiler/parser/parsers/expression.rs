use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Expression, Operator, Value},
};

pub fn parse_expression(tokens: &mut Tokens, required: bool) -> Option<Expression> {
    tokens.peek_expect_token(vec![]);

    let expression = tokens.create_expression(match info.token.clone() {
        Token::Integer(integer) => Expression::Value(Value::Integer {
            minus: false,
            integer: integer.clone(),
        }),
        Token::Identifier(name) => Expression::GetVariable(name),
        token => {
            if required {
                tokens.throw_error(format!("Expected expression, got '{}'", token), "");
            }
            return None;
        }
    });

    let info = match tokens.peek_expect_tokens(vec![
        Token::Plus,
        Token::Minus,
        Token::ForwardSlash,
        Token::Asterisk,
    ]) {
        Some(info) => info,
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

    Some(Expression::BinaryOperation(
        Box::new(expression),
        operator,
        Box::new(second_expression),
    ))
}
