use crate::{lexer::TokenInfo, CompileError};

use super::{
    after_identifier::{parse_after_identifier, parse_identifer_string},
    enums::parse_enum,
    export::parse_export,
    expression::parse_expression,
    function::parse_function,
    node::*,
    scope::parse_scope,
    structs::parse_struct,
    variable::parse_define_variable,
};
use crate::lexer::{Token, TokensGroup};

pub fn parse(tokens: &mut TokensGroup) -> Result<Vec<ASTNode>, CompileError> {
    let mut tree: Vec<ASTNode> = Vec::new();

    loop {
        let info = tokens.peek()?;
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
            Token::StartScope => {
                let body = parse_scope(tokens)?;
                Ok(tokens.generate(Node::Scope {
                    is_unsafe: false,
                    body,
                }))
            }
            Token::Import => {
                let name = parse_identifer_string(tokens)?;
                Ok(tokens.generate(Node::Import(name, false)))
            }
            //--------------[[Function]]--------------
            Token::Pub => parse_export(tokens),
            Token::Unsafe => match tokens.advance() {
                Ok(info) => match info.token {
                    Token::Function => parse_function(tokens, false, true),
                    Token::StartScope => {
                        let body = parse_scope(tokens)?;

                        Ok(tokens.generate(Node::Scope {
                            is_unsafe: true,
                            body,
                        }))
                    }
                    _ => return Err(tokens_expected_got(tokens, vec![Token::Function], info)),
                },
                Err(error) => return Err(error),
            },
            Token::Struct => parse_struct(tokens, false),
            Token::Enum => parse_enum(tokens, false),
            Token::Function => parse_function(tokens, false, false),
            Token::Return => {
                let expression = parse_expression(tokens)?;

                let info = tokens.advance()?;
                match info.token {
                    Token::SemiColon => {}
                    _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
                }

                Ok(tokens.generate(Node::Return(expression)))
            }
            //--------------[[FUNCTION-END]]--------------
            Token::Loop => {
                let info = tokens.advance()?;
                match info.token {
                    Token::StartScope => {}
                    _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
                }

                let body = parse_scope(tokens)?;

                Ok(tokens.generate(Node::Loop { body: body }))
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
    _tokens: &TokensGroup,
    _expected: Vec<Token>,
    _got: TokenInfo,
) -> CompileError {
    return CompileError;
}
