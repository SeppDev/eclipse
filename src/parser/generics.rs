use crate::{
    analyzer::Fields,
    lexer::{Token, TokensGroup},
    CompileError, ParseResult,
};

use super::parser::expect_tokens;

pub fn parse_generics(tokens: &mut TokensGroup) -> ParseResult<Vec<String>> {
    let mut generics = Vec::new();
    let mut fields = Fields::new();

    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(name) => {
                fields.insert(name.clone(), info.line)?;
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
