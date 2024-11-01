use crate::{
    lexer::{Token, TokensGroup}, types::Path, ParseResult
};


pub fn parse_path(tokens: &mut TokensGroup, root: String) -> ParseResult<Path> {
    let mut path = Path::from(root);
    loop {
        let info = tokens.peek()?;
        match info.token {
            Token::Identifier(name) => path.add(name),
            Token::DoubleColon => {}
            _ => break,
        }
        tokens.advance()?;
    }

    return Ok(path);
}
