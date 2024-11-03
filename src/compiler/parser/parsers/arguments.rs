use crate::compiler::{
    lexer::{Token, Tokens},
    parser::Expression,
};

use super::expression::parse_expression;

pub fn parse_arguments(tokens: &mut Tokens) -> Vec<Expression> {
    let mut arguments = Vec::new();

    loop {
        if tokens.peek_expect_token(Token::CloseParen) {
            break;
        };
        arguments.push(parse_expression(tokens, true).unwrap());
        match tokens
            .expect_tokens(vec![Token::Comma, Token::CloseParen])
            .token
        {
            Token::Comma => continue,
            Token::CloseParen => break,
            _ => panic!(),
        }
    }

    return arguments;
}
