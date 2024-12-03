use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{expression::parse_expression, types::parse_type};

pub fn parse_variable(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let mutable = tokens
        .peek_expect_tokens(vec![Token::Mutable], true)
        .is_some();
    let name = tokens.parse_identifier()?;

    let data_type = if tokens
        .peek_expect_tokens(vec![Token::Colon], true)
        .is_some()
    {
        Some(parse_type(tokens)?)
    } else {
        None
    };


    let expression = if tokens
        .peek_expect_tokens(vec![Token::Equals], true)
        .is_some()
    {
        parse_expression(tokens, false)?
    } else {
        None
    };

    return Ok(tokens.create_node(Node::DeclareVariable {
        name,
        mutable,
        data_type,
        expression,
    }));
}

pub fn parse_set_variable(tokens: &mut Tokens, name: String) -> CompileResult<NodeInfo> {
    let expression = parse_expression(tokens, true)?;
    return Ok(tokens.create_node(Node::SetVariable { name, expression }));
}
