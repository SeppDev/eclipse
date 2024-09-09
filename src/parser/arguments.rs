use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{expression::parse_expression, tokens_expected_got, Expression};

pub fn parse_arguments(tokens: &mut TokensGroup) -> Result<Vec<Expression>, CompileError> {
    let mut expressions = Vec::new();
    loop {
        match parse_expression(tokens) {
            Ok(expression) => match expression {
                Some(expression) => expressions.push(expression),
                None => break,
            },
            Err(error) => return Err(error),
        };
        match tokens.advance() {
            Ok(info) => match info.token {
                Token::Comma => continue,
                Token::CloseParen => break,
                _ => {
                    return Err(tokens_expected_got(
                        tokens,
                        vec![Token::Comma, Token::CloseParen],
                        info,
                    ))
                }
            },
            Err(error) => return Err(error),
        }
    }
    return Ok(expressions);
}
