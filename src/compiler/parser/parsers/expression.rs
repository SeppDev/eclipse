use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Expression, Value},
};

pub fn parse_expression(tokens: &mut Tokens, required: bool) -> Option<Expression> {
    let info = tokens.start();

    let expression = match info.token.clone() {
        Token::Integer(integer) => Expression::Value(Value::Integer {
            minus: false,
            integer: integer.clone(),
        }),
        Token::Identifier(name) => {
            
        }
        token => {
            if required {
                tokens.throw_error(format!("Expected expression, got '{}'", token), "");
            }
            return None;
        }
    };

    Some(tokens.create_expression(expression))
}
