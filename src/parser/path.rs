use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::Path;

pub fn parse_path(tokens: &mut TokensGroup, root: String) -> Result<Path, BuildError> {
    let mut path = Path::new(root);

    loop {
        let info = tokens.peek()?;
        match info.token {
            Token::DoubleColon => {
                tokens.advance()?;
            }
            _ => break,
        }
        
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(name) => path.add(name),
            _ => {}
        };
    }

    return Ok(path);
}
