use crate::compiler::{
    lexer::{Token, Tokens},
    parser::ExpressionInfo,
};

use super::expression::parse_expression;

pub fn parse_arguments(tokens: &mut Tokens) -> Vec<ExpressionInfo> {
    let mut arguments = Vec::new();

    loop {
        if tokens.peek_expect_tokens(vec![Token::CloseParen], true).is_some() {
            break;
        };
        
        arguments.push(parse_expression(tokens, true).unwrap());
        match tokens
            .expect_tokens(vec![Token::Comma, Token::CloseParen], false)
            .token
        {
            Token::Comma => continue,
            Token::CloseParen => break,
            _ => panic!(),
        }
    }

    return arguments;
}
