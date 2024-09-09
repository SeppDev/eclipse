use eclipse::ParseError;

use crate::parser::{
    lexer::Token,
    node::{self, Expression, Value},
    tokens_group::TokensGroup,
};

use super::call::call_function;

pub fn result(tokens: &mut TokensGroup) -> Result<Option<Expression>, ParseError> {
    let _is_reference = match tokens.peek().unwrap().token {
        Token::Reference => {
            tokens.next_token();
            true
        }
        _ => false,
    };

    use crate::parser::Type;
    let mut expression = match tokens.peek().unwrap().token {
        Token::Integer(integer) => {
            tokens.next_token();
            Expression::Value(Value::Integer(integer), Type::Integer(node::Integer::i64))
        }
        Token::String(string) => {
            tokens.next_token();
            Expression::Value(Value::String(string), Type::String)
        }
        Token::Boolean(boolean) => {
            tokens.next_token();
            Expression::Value(Value::Boolean(boolean), Type::Boolean)
        }
        Token::Identifier(name) => {
            tokens.next_token();
            let tokeninfo = match tokens.peek() {
                Some(t) => t,
                None => return Err(ParseError::NoTokenFound),
            };

            match tokeninfo.token {
                Token::OpenParen => {
                    tokens.next_token();
                    Expression::Call(
                        name,
                        match call_function(tokens) {
                            Ok(a) => a,
                            Err(error) => return Err(error),
                        },
                    )
                }
                _ => Expression::GetVariable(name.clone()),
            }
        }
        _ => return Ok(None),
    };

    match tokens.peek().unwrap().token {
        Token::Operator(operator) => {
            use crate::parser::lexer::Operator;

            tokens.next_token();
            let node_operator: node::Operator = match operator {
                Operator::Plus => node::Operator::Plus,
                Operator::Minus => node::Operator::Minus,
                Operator::Division => node::Operator::Division,
                Operator::Multiply => node::Operator::Multiply,
            };
            expression = Expression::BinaryOperation(
                Box::new(expression),
                node_operator,
                Box::new(match result(tokens) {
                    Ok(a) => match a {
                        Some(a) => a,
                        None => return Err(ParseError::NoTokenFound),
                    },
                    Err(error) => return Err(error),
                }),
            );
        }
        _ => {}
    }

    return Ok(Some(expression));
}
