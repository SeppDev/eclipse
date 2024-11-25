use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::ExpressionInfo,
};

use super::expression::parse_expression;

pub fn parse_arguments(tokens: &mut Tokens) -> CompileResult<Vec<ExpressionInfo>> {
    let mut arguments = Vec::new();

    loop {
        if tokens
            .peek_expect_tokens(vec![Token::CloseParen], true)
            .is_some()
        {
            break;
        };
        let expression = match parse_expression(tokens, true)? {
            Some(e) => e,
            None => break
        };

        arguments.push(expression);
        let result = tokens.expect_tokens(vec![Token::Comma, Token::CloseParen], false);
        match result.token {
            Token::Comma => continue,
            Token::CloseParen => break,
            _ => panic!(),
        }
    }

    return Ok(arguments);
}
