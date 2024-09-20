use crate::{
    lexer::{Token, TokensGroup},
    parser::parser::get_identifier,
    ParseResult,
};

use super::{
    expression::parse_expected_expression,
    node::{ASTNode, Node, Type},
    parser::{expect_tokens, peek_expect_tokens},
    types::parse_type,
};

pub fn parse_variable(tokens: &mut TokensGroup) -> ParseResult<ASTNode> {
    let name = get_identifier(tokens)?;
    let mutable = peek_expect_tokens(tokens, vec![Token::Mutable], true).is_ok();

    let data_type: Option<Type> = if peek_expect_tokens(tokens, vec![Token::Colon], true).is_ok() {
        Some(parse_type(tokens)?)
    } else {
        None
    };

    expect_tokens(tokens, vec![Token::Equals])?;

    let expression = parse_expected_expression(tokens)?;

    return Ok(tokens.create_AST(Node::DefineVariable {
        mutable,
        name,
        data_type,
        expression: Some(expression),
    }));
}
