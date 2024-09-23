use crate::{
    lexer::{Token, TokensGroup},
    CompileError, ParseResult,
};

use super::parser::expect_tokens;

pub fn parse_struct(tokens: &mut TokensGroup) -> ParseResult<Vec<String>> {
    let mut generics = Vec::new();

    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(name) => {
                generics.push(name);
                let info = expect_tokens(tokens, vec![Token::Comma, Token::GreaterThan])?;
                if info.token == Token::GreaterThan {
                    break;
                }
            }
            Token::GreaterThan => break,
            token => {
                return Err(CompileError::new(
                    format!("Expected genertic identifier, got: {:?}", token),
                    info.line,
                ))
            }
        }
    }

    return Ok(generics);
}
