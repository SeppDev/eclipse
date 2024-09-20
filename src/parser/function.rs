use crate::{
    lexer::{Token, TokensGroup},
    parser::parser::get_identifier,
    ParseResult,
};

use super::{
    node::{ASTNode, Node, Type},
    parse,
    parser::{expect_tokens, peek_expect_tokens},
    types::parse_type,
};

pub fn parse_function(
    tokens: &mut TokensGroup,
    is_unsafe: bool,
    export: bool,
) -> ParseResult<ASTNode> {
    let name = get_identifier(tokens)?;
    expect_tokens(tokens, vec![Token::OpenParen])?;

    let parameters = Vec::new();
    loop {
        let info = tokens.advance()?;
        match info.token {
            Token::Identifier(_name) => {}
            Token::CloseParen => break,
            _ => todo!(),
        }
    }

    let return_type: Option<Type> = if peek_expect_tokens(tokens, vec![Token::Colon], true).is_ok()
    {
        Some(parse_type(tokens)?)
    } else {
        None
    };

    expect_tokens(tokens, vec![Token::StartScope])?;
    let body = parse(tokens)?;

    Ok(ASTNode::new(
        tokens.start.line..tokens.current.line,
        Node::Function {
            export,
            is_unsafe,
            name,
            parameters,
            return_type,
            body,
        },
    ))
}
