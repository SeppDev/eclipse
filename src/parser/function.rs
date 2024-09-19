use crate::{
    lexer::{Token, TokensGroup}, BuildError, CompileError
};

use super::{
    after_identifier::parse_identifer_string,
    node::{ASTNode, Node},
    scope::parse_scope,
    tokens_expected_got,
    types::parse_type,
    Type,
};

pub fn parse_function(
    tokens: &mut TokensGroup,
    export: bool,
    is_unsafe: bool,
) -> Result<ASTNode, BuildError> {
    let name = parse_identifer_string(tokens)?;

    let info = tokens.advance()?;
    match info.token {
        Token::OpenParen => {}
        _ => return Err(tokens_expected_got(tokens, vec![Token::OpenParen], info)),
    }

    let mut parameters = Vec::new();
    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(name) => {
                parameters.push((name, parse_type(tokens)?));
                let info = tokens.advance()?;
                match info.token {
                    Token::Comma => {}
                    Token::CloseParen => break,
                    _ => return Err(tokens_expected_got(tokens, vec![Token::Comma], info)),
                }
            }
            Token::CloseParen => break,
            _ => return Err(tokens_expected_got(tokens, vec![Token::OpenParen], info)),
        }
    }

    let info = tokens.advance()?;
    let return_type: Option<Type> = match info.token {
        Token::StartScope => None,
        Token::Colon => {
            let data_type = parse_type(tokens)?;
            let info = tokens.advance()?;
            match info.token {
                Token::StartScope => {}
                _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
            }
            Some(data_type)
        }
        _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
    };

    let body = parse_scope(tokens)?;

    return Ok(tokens.generate(Node::Function {
        export,
        is_unsafe,
        name,
        parameters,
        return_type: return_type,
        body: body,
    }));
}
