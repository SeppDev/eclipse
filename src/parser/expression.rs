use crate::{
    lexer::{Token, TokensGroup},
    CompileError, ParseResult,
};

use super::node::{ASTNode, Expression};

pub fn parse_expression(tokens: &mut TokensGroup) -> ParseResult<Option<Expression>> {
    let info = tokens.peek()?;
    match info.token {
        Token::SemiColon => return Ok(None),
        _ => {}
    };

    
}

pub fn parse_expected_expression(tokens: &mut TokensGroup) -> ParseResult<Expression> {
    return match parse_expression(tokens)? {
        Some(expr) => Ok(expr),
        None => Err(CompileError::new(
            format!("Expected expression"),
            tokens.current.line,
        )),
    };
}
