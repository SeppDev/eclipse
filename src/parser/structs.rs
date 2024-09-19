use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{
    after_identifier::parse_identifer_string, tokens_expected_got, types::parse_type, ASTNode, Node,
};

pub fn parse_struct(tokens: &mut TokensGroup, export: bool) -> Result<ASTNode, BuildError> {
    let name = parse_identifer_string(tokens)?;

    let info = tokens.advance()?;
    match info.token {
        Token::StartScope => {}
        _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
    }

    let mut body = Vec::new();

    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::EndScope => break,
            Token::Comma => continue,
            Token::Pub => body.push((true, parse_identifer_string(tokens)?, parse_type(tokens)?)),
            Token::Identifier(name) => body.push((false, name, parse_type(tokens)?)),
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("struct"))],
                    info,
                ))
            }
        }
    }

    return Ok(tokens.generate(Node::Struct {
        export,
        name,
        generics: Vec::new(),
        body: body,
    })?);
}
