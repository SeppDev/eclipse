use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{
    after_identifier::parse_identifer_string, tokens_expected_got, types::parse_type, ASTNode, Node,
};

pub fn parse_enum(tokens: &mut TokensGroup, export: bool) -> Result<ASTNode, CompileError> {
    let line = tokens.current.lines.clone();

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
            Token::Identifier(name) => {
                let mut types = Vec::new();

                let info = match tokens.peek() {
                    Ok(info) => info,
                    Err(error) => return Err(error),
                };
                match info.token {
                    Token::OpenParen => {
                        tokens.advance().unwrap();
                        loop {
                            types.push(match parse_type(tokens) {
                                Ok(t) => t,
                                Err(error) => return Err(error),
                            });
                            let info = match tokens.advance() {
                                Ok(info) => info,
                                Err(error) => return Err(error),
                            };
                            match info.token {
                                Token::Comma => continue,
                                Token::CloseParen => break,
                                _ => {
                                    return Err(tokens_expected_got(
                                        tokens,
                                        vec![Token::Comma],
                                        info,
                                    ))
                                }
                            }
                        }
                    }
                    _ => {}
                }

                body.push((name, types))
            }
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("enum"))],
                    info,
                ))
            }
        }
    }

    return Ok(ASTNode::new(
        line,
        Node::Enum {
            export,
            name,
            generics: Vec::new(),
            body: body,
        },
    ));
}
