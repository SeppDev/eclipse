use std::collections::HashMap;

use crate::{
    lexer::{Token, TokenInfo, TokensGroup},
    CompileError, ParseResult,
};

use super::{
    enums::parse_enum, expression::parse_expression, function::parse_function,
    identifier::parse_identifier, node::ASTNode, structs::parse_struct, variable::parse_variable,
    Node,
};

pub fn get_identifier(tokens: &mut TokensGroup) -> ParseResult<String> {
    let info = tokens.advance()?;

    match info.token {
        Token::Identifier(name) => return Ok(name),
        token => {
            return Err(CompileError::new(
                format!("Expected identifier got: {:?}", token),
                tokens.current.line,
            ))
        }
    }
}

pub fn parse(tokens: &mut TokensGroup) -> ParseResult<(Vec<ASTNode>, Vec<(bool, String)>)> {
    let mut tree = Vec::new();
    let mut imports = Vec::new();

    loop {
        if tokens.is_eof()? == true {
            break;
        }
        peek_expect_tokens(tokens, vec![Token::SemiColon], true)
            .is_ok()
            .then(|| {});

        let info = tokens.peek()?;
        match info.token {
            Token::EndScope => break,
            _ => {}
        }

        let info = tokens.start()?;

        let node: ASTNode = match info.token {
            Token::EndOfFile => break,
            Token::Import => {
                let name = get_identifier(tokens)?;
                imports.push((false, name));
                continue;
            }
            Token::StartScope => {
                let (body, _) = parse(tokens)?;
                expect_tokens(tokens, vec![Token::EndScope])?;
                tokens.create_ast(Node::Scope {
                    is_unsafe: false,
                    body,
                })
            }
            Token::Pub => {
                let info = tokens.advance()?;
                match info.token {
                    Token::Function => parse_function(tokens, false, true)?,
                    Token::Import => {
                        let name = get_identifier(tokens)?;
                        imports.push((true, name));
                        continue;
                    }
                    _ => panic!(),
                }
            }
            Token::Function => parse_function(tokens, false, false)?,
            Token::Identifier(string) => parse_identifier(tokens, string)?,
            Token::Variable => parse_variable(tokens)?,
            Token::Struct => parse_struct(tokens)?,
            Token::Enum => parse_enum(tokens)?,
            Token::Loop => {
                expect_tokens(tokens, vec![Token::StartScope])?;
                let (body, _) = parse(tokens)?;
                expect_tokens(tokens, vec![Token::EndScope])?;
                tokens.create_ast(Node::Loop(body))
            }
            Token::Return => {
                let expression = parse_expression(tokens)?;
                tokens.create_ast(Node::Return(expression))
            }
            token => {
                return Err(CompileError::new(
                    format!("Unhandled token: {:?}", token),
                    tokens.current.line,
                ))
            }
        };

        tree.push(node);
    }

    if imports.len() > 0 {
        if tokens.indent > 0 {
            return Err(tokens.create_error(format!("Cannot import inline")));
        }
        let mut map: HashMap<&String, &bool> = HashMap::new();
        for (public, key) in &imports {
            match map.insert(key, public) {
                Some(key) => {
                    return Err(CompileError::new(
                        format!("{:?} is already imported", key),
                        0,
                    ))
                }
                _ => continue,
            }
        }
    }

    return Ok((tree, imports));
}

pub fn expect_tokens(tokens: &mut TokensGroup, expected: Vec<Token>) -> ParseResult<TokenInfo> {
    let info = tokens.advance()?;
    let mut found = false;
    for token in &expected {
        if token == &info.token {
            found = true;
            break;
        }
    }

    if found {
        return Ok(info);
    }
    return Err(CompileError::new(
        format!("Expected tokens: {:?}, got: {:?}", expected, info),
        tokens.current.line,
    ));
}
pub fn peek_expect_tokens(
    tokens: &mut TokensGroup,
    expected: Vec<Token>,
    advance_if_found: bool,
) -> ParseResult<Option<TokenInfo>> {
    let info = tokens.peek()?;
    let mut found = false;
    for token in &expected {
        if token == &info.token {
            found = true;
            break;
        }
    }

    if found {
        if advance_if_found {
            tokens.advance()?;
        }
        return Ok(Some(info));
    }
    return Ok(None);
    // return Err(CompileError::new(
    //     format!("Expected tokens: {:?}, got: {:?}", expected, info),
    //     tokens.current.line,
    // ));
}
