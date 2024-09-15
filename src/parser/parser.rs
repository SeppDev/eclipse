use crate::{lexer::TokenInfo, BuildError, BuildProblem, CompileError};

use super::{
    after_identifier::{parse_after_identifier, parse_identifer_string}, enums::parse_enum, export::parse_export,
    expression::parse_expression, function::parse_function, node::*, scope::parse_scope,
    structs::parse_struct, variable::parse_define_variable,
};
use crate::lexer::{Token, TokensGroup};

pub fn parse(tokens: &mut TokensGroup) -> Result<Vec<ASTNode>, CompileError> {
    let mut tree: Vec<ASTNode> = Vec::new();

    loop {
        let info = match tokens.peek() {
            Ok(info) => info,
            Err(error) => return Err(error),
        };

        match info.token {
            Token::EndOfFile => break,
            Token::EndScope => break,
            _ => {}
        }

        let info = match tokens.advance() {
            Ok(info) => info,
            Err(error) => return Err(error),
        };

        let node = match info.token {
            Token::EndOfFile => break,
            Token::SemiColon => continue,
            Token::Variable => parse_define_variable(tokens),
            Token::Identifier(name) => parse_after_identifier(tokens, name),
            Token::StartScope => Ok(ASTNode::new(
                tokens.current.lines.clone(),
                Node::Scope {
                    is_unsafe: false,
                    body: match parse_scope(tokens) {
                        Ok(body) => body,
                        Err(error) => return Err(error),
                    },
                },
            )),
            Token::Import => {
                let name = match parse_identifer_string(tokens) {
                    Ok(str) => str,
                    Err(error) => return Err(error)
                };
                Ok(ASTNode::new(tokens.current.lines.clone(), Node::Import(name, false)))
            },
            //--------------[[Function]]--------------
            Token::Pub => parse_export(tokens),
            Token::Unsafe => match tokens.advance() {
                Ok(info) => match info.token {
                    Token::Function => parse_function(tokens, false, true),
                    Token::StartScope => Ok(ASTNode::new(
                        tokens.current.lines.clone(),
                        Node::Scope {
                            is_unsafe: true,
                            body: match parse_scope(tokens) {
                                Ok(body) => body,
                                Err(error) => return Err(error),
                            },
                        },
                    )),
                    _ => return Err(tokens_expected_got(tokens, vec![Token::Function], info)),
                },
                Err(error) => return Err(error),
            },
            Token::Struct => parse_struct(tokens, false),
            Token::Enum => parse_enum(tokens, false),
            Token::Function => parse_function(tokens, false, false),
            Token::Return => {
                let expression = match parse_expression(tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };

                match tokens.advance() {
                    Ok(info) => match info.token {
                        Token::SemiColon => {}
                        _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
                    },
                    Err(error) => return Err(error),
                }

                Ok(ASTNode::new(tokens.current.lines.clone(), Node::Return(expression)))
            }
            //--------------[[FUNCTION-END]]--------------
            // Token::OpenParen
            Token::Loop => {
                match tokens.advance() {
                    Ok(info) => match info.token {
                        Token::StartScope => {}
                        _ => {
                            return Err(tokens_expected_got(tokens, vec![Token::StartScope], info))
                        }
                    },
                    Err(error) => return Err(error),
                }

                let body = match parse_scope(tokens) {
                    Ok(body) => body,
                    Err(error) => return Err(error),
                };

                Ok(ASTNode::new(tokens.current.lines.clone(), Node::Loop { body: body }))
            }
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![
                        Token::EndOfFile,
                        Token::Identifier(String::from("name")),
                        Token::Function,
                        Token::Import,
                    ],
                    info,
                ))
            }
        };
        match node {
            Ok(node) => tree.push(node),
            Err(error) => return Err(error),
        }
    }

    return Ok(tree);
}

pub fn tokens_expected_got(
    tokens: &TokensGroup,
    expected: Vec<Token>,
    got: TokenInfo,
) -> CompileError {
    return CompileError::BuildProblem(BuildProblem::new(
        BuildError::TokensExpectedGot(expected, got),
        tokens.relative_path.clone(),
        tokens.current.lines.clone(),
    ));
}
