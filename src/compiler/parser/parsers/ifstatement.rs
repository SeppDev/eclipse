use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{body::parse_body, expression::parse_expression};
pub fn parse_ifstatement(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let expression = parse_expression(tokens, true)?.unwrap();
    tokens.expect_tokens(vec![Token::StartScope], false)?;

    let body = parse_body(tokens)?;

    let else_body = if tokens.peek_expect_tokens(vec![Token::Else], true).is_some() {
        tokens.expect_tokens(vec![Token::StartScope], false)?;
        let body = parse_body(tokens)?;
        Some(body)
    } else {
        None
    };

    return Ok(tokens.create_node(Node::IfStatement {
        expression,
        body,
        elseif: Vec::new(),
        else_body,
    }));
}
