use crate::{
    lexer::{Token, TokensGroup},
    ParseResult,
};

use super::{
    generics::parse_generics, get_identifier, parser::expect_tokens, peek_expect_tokens,
    types::parse_type, ASTNode, Node, Type,
};

pub fn parse_struct(tokens: &mut TokensGroup) -> ParseResult<ASTNode> {
    let name = get_identifier(tokens)?;
    expect_tokens(tokens, vec![Token::StartScope])?;

    let generics = if peek_expect_tokens(tokens, vec![Token::LessThan], true)?.is_some() {
        Some(parse_generics(tokens)?)
    } else {
        None
    };

    let mut body: Vec<(bool, String, Type)> = Vec::new();
    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::EndScope => break,
            Token::Identifier(name) => {
                body.push((false, name, parse_type(tokens)?));
            }
            _ => todo!(),
        }
    }

    return Ok(tokens.create_ast(Node::Struct {
        export: false,
        name,
        generics,
        body,
    }));
}
