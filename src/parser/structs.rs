use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{after_identifier::parse_identifer_string, tokens_expected_got, types::parse_type, ASTNode, Node};

pub fn parse_struct(tokens: &mut TokensGroup, export: bool) -> Result<ASTNode, CompileError> {
    let line = tokens.current.line;

    let name = match parse_identifer_string(tokens) {
        Ok(str) => str,
        Err(error) => return Err(error),
    };

    match tokens.advance() {
        Ok(info) => match info.token {
            Token::StartScope => {}
            _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
        },
        Err(error) => return Err(error),
    };

    let mut body = Vec::new();

    loop {
        let info = match tokens.advance() {
            Ok(info) => info,
            Err(error) => return Err(error),
        };
        match info.token {
            Token::EndScope => break,
            Token::Comma => continue,
            Token::Pub => {
                let name = match parse_identifer_string(tokens) {
                    Ok(str) => str,
                    Err(error) => return Err(error),
                };
                let t = match parse_type(tokens) {
                    Ok(t) => t,
                    Err(error) => return Err(error)
                };

                body.push((true, name, t))
            },
            Token::Identifier(name) => {
                let t = match parse_type(tokens) {
                    Ok(t) => t,
                    Err(error) => return Err(error)
                };

                body.push((false, name, t))
            }
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("struct"))],
                    info,
                ))
            }
        }
    }

    return Ok(ASTNode::new(
        line,
        Node::Struct {
            export,
            name,
            generics: Vec::new(),
            body: body,
        },
    ));
}
