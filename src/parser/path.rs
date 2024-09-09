use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::Path;

pub fn parse_path(tokens: &mut TokensGroup, root: String) -> Result<Path, CompileError> {
    let mut path = Path::new(root);

    loop {
        match tokens.peek() {
            Ok(info) => match info.token {
                Token::DoubleColon => {
                    tokens.advance().unwrap();
                }
                // Token::OpenParen => break,
                // Token::CloseParen => break,
                // Token::Comma => break,
                // Token::SemiColon => break,
                _ => break,
            },
            Err(error) => return Err(error),
        }
        match tokens.advance() {
            Ok(info) => match info.token {
                Token::Identifier(name) => path.add(name),
                _ => {}
            },
            Err(error) => return Err(error),
        }
    }

    return Ok(path);
}
