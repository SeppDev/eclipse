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
    
    let generics = if peek_expect_tokens(tokens, vec![Token::LessThan], true)?.is_some() {
        Some(parse_generics(tokens)?)
    } else {
        None
    };
    expect_tokens(tokens, vec![Token::StartScope])?;
    
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
        let info = expect_tokens(tokens, vec![Token::Comma, Token::EndScope])?;
        match info.token {
            Token::Comma => continue,
            _ => break,
        }
    }

    return Ok(tokens.create_ast(Node::Struct {
        export: false,
        name,
        generics,
        body,
    }));
}
