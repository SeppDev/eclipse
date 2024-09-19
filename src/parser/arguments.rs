use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{expression::parse_expression, tokens_expected_got, Expression};

pub fn parse_arguments(tokens: &mut TokensGroup) -> Result<Vec<Expression>, BuildError> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(match parse_expression(tokens)? {
            Some(e) => e,
            None => break,
        });
        let info = tokens.advance()?;
        match info.token {
            Token::Comma => continue,
            Token::CloseParen => break,
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Comma, Token::CloseParen],
                    info,
                ))
            }
        };
    }
    return Ok(expressions);
}
