use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{
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
) -> Result<ASTNode, CompileError> {
    let name = match tokens.advance() {
        Ok(info) => match info.token {
            Token::Identifier(name) => name,
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("function_name"))],
                    info,
                ))
            }
        },
        Err(error) => return Err(error),
    };

    match tokens.advance() {
        Ok(info) => match info.token {
            Token::OpenParen => {}
            _ => return Err(tokens_expected_got(tokens, vec![Token::OpenParen], info)),
        },
        Err(error) => return Err(error),
    }

    let mut parameters = Vec::new();
    loop {
        match tokens.advance() {
            Ok(info) => match info.token {
                Token::Identifier(name) => {
                    parameters.push((
                        name,
                        match parse_type(tokens) {
                            Ok(t) => t,
                            Err(error) => return Err(error),
                        },
                    ));
                    match tokens.advance() {
                        Ok(info) => match info.token {
                            Token::Comma => {}
                            Token::CloseParen => break,
                            _ => return Err(tokens_expected_got(tokens, vec![Token::Comma], info)),
                        },
                        Err(error) => return Err(error),
                    }
                }
                Token::CloseParen => break,
                _ => return Err(tokens_expected_got(tokens, vec![Token::OpenParen], info)),
            },
            Err(error) => return Err(error),
        }
    }

    let return_type: Option<Type> = match tokens.advance() {
        Ok(info) => match info.token {
            Token::StartScope => None,
            Token::Colon => match parse_type(tokens) {
                Ok(t) => {
                    match tokens.advance() {
                        Ok(info) => match info.token {
                            Token::StartScope => {}
                            _ => {
                                return Err(tokens_expected_got(
                                    tokens,
                                    vec![Token::StartScope],
                                    info,
                                ))
                            }
                        },
                        Err(error) => return Err(error),
                    };

                    Some(t)
                }
                Err(error) => return Err(error),
            },
            _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
        },
        Err(error) => return Err(error),
    };

    let body = match parse_scope(tokens) {
        Ok(body) => body,
        Err(error) => return Err(error),
    };

    return Ok(ASTNode::new(
        tokens.current.line,
        Node::Function {
            export,
            is_unsafe,
            name,
            parameters,
            return_type: return_type,
            body: body,
        },
    ));
}
