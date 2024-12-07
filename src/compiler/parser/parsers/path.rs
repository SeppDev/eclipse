use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    path::Path,
};

pub fn parse_path(tokens: &mut Tokens, root: &String) -> CompileResult<Path> {
    let mut path = Path::from(root);
    loop {
        if !tokens
            .peek_expect_tokens(vec![Token::DoubleColon], true)
            .is_some()
        {
            break;
        }
        let info = tokens.expect_tokens(vec![Token::Identifier(String::new())], false)?;
        match info.token {
            Token::Identifier(name) => path.push(name),
            _ => panic!(),
        }
    }

    return Ok(path);
}
