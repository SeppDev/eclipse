use crate::{
    lexer::{Token, TokensGroup},
    ParseResult,
};

use super::{expression::parse_expression, node::Expression, parser::expect_tokens};

pub fn parse_arguments(tokens: &mut TokensGroup) -> ParseResult<Vec<Expression>> {
    let mut arguments = Vec::new();

    loop {
        let expression = match parse_expression(tokens)? {
            Some(expr) => expr,
            None => break,
        };
        arguments.push(expression);

        let info = expect_tokens(tokens, vec![Token::CloseParen, Token::Comma])?;
        match info.token {
            Token::Comma => continue,
            Token::CloseParen => break,
            _ => panic!(),
        }
    }

    return Ok(arguments);
}
