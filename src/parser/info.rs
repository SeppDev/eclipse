use crate::{lexer::{Token, TokensGroup}, CompileError};


pub fn check_export(tokens: &mut TokensGroup) -> Result<bool, CompileError> {
    let mut history = tokens.previous_tokens(1);
    return match history.pop() {
        Some(info) => match info.token {
            Token::Export => Ok(true),
            _ => Ok(false)
        },
        None => return Ok(true)
    };
}