use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{body::parse_body, expression::parse_expression};

pub fn parse_loop(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let _ = tokens.expect_tokens(vec![Token::StartScope], false);

    let body = parse_body(tokens)?;

    return Ok(tokens.create_node(Node::Loop {
        condition: None,
        body,
    }));
}


pub fn parse_while(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let expression = parse_expression(tokens, true)?.unwrap();
    let _ = tokens.expect_tokens(vec![Token::StartScope], false);

    let body = parse_body(tokens)?;

    return Ok(tokens.create_node(Node::Loop {
        condition: Some(expression),
        body,
    }));
}
