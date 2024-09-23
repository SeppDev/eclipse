use crate::{
    lexer::{Token, TokensGroup},
    parser::parser::get_identifier,
    ParseResult,
};

use super::{
    generics::parse_generics, node::{ASTNode, Node, Type}, parse, parser::{expect_tokens, peek_expect_tokens}, types::parse_type
};

pub fn parse_function(
    tokens: &mut TokensGroup,
    is_unsafe: bool,
    export: bool,
) -> ParseResult<ASTNode> {
    let name = get_identifier(tokens)?;
    let info = peek_expect_tokens(tokens, vec![Token::LessThan], true)?;

    let mut generics = Vec::new();
    if info.is_some() {
        generics = parse_generics(tokens)?;
    }

    expect_tokens(tokens, vec![Token::OpenParen])?;
    let mut parameters = Vec::new();
    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(name) => {
                let data_type = parse_type(tokens)?;
                parameters.push((name, data_type));
                let info = expect_tokens(tokens, vec![Token::Comma, Token::CloseParen])?;
                match info.token {
                    Token::CloseParen => break,
                    _ => {}
                }
            }
            Token::CloseParen => break,
            token => todo!("{:?}", token),
        }
    }

    let mut return_type: Option<Type> = None;
    if peek_expect_tokens(tokens, vec![Token::Colon], true)?.is_some() {
        return_type = Some(parse_type(tokens)?);
    }

    expect_tokens(tokens, vec![Token::StartScope])?;
    let body = parse(tokens)?;

    expect_tokens(tokens, vec![Token::EndScope])?;

    return Ok(tokens.create_ast(
        Node::Function {
            export,
            is_unsafe,
            name,
            generics,
            parameters,
            return_type,
            body,
        },
    ));
}
