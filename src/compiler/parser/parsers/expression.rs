use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Expression, Value},
};

pub fn parse_expression(tokens: &mut Tokens, required: bool) -> Option<Expression> {
    let info = tokens.peek();

    let expression = match &info.token {
        Token::Integer(integer) => Expression::Value(Value::Integer {
            minus: false,
            integer: integer.clone(),
        }),
        token => {
            if required {
                tokens.throw_error(format!("Expected expression, got '{}'", token), "");
            }
            return None;
        }
    };
    tokens.advance();

    Some(expression)
}
