use crate::{
    analyzer::Fields, lexer::{Token, TokensGroup}, types::{ASTNode, Node, Type}, ParseResult
};

use super::{
    generics::parse_generics, get_identifier, parser::expect_tokens, peek_expect_tokens,
    types::parse_type
};

pub fn parse_enum(tokens: &mut TokensGroup) -> ParseResult<ASTNode> {
    let name = get_identifier(tokens)?;

    let generics = if peek_expect_tokens(tokens, vec![Token::LessThan], true)?.is_some() {
        Some(parse_generics(tokens)?)
    } else {
        None
    };
    expect_tokens(tokens, vec![Token::StartScope])?;

    let mut fields = Fields::new();
    let mut body: Vec<(String, Option<Type>)> = Vec::new();

    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::EndScope => break,
            Token::Identifier(name) => {
                fields.insert(name.clone(), info.line)?;
                let types = if peek_expect_tokens(tokens, vec![Token::OpenParen], false)?.is_some()
                {
                    Some(parse_type(tokens)?)
                } else {
                    None
                };

                body.push((name, types));
            }
            _ => todo!(),
        }
        let info = expect_tokens(tokens, vec![Token::Comma, Token::EndScope])?;
        match info.token {
            Token::Comma => continue,
            _ => break,
        }
    }

    return Ok(tokens.create_ast(Node::Enum {
        export: false,
        name,
        generics,
        body,
    }));
}
