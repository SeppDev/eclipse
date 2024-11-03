use crate::compiler::{
    lexer::{Token, Tokens},
    parser::Node,
    types::{BaseType, Type},
};

use super::expression::parse_expression;

pub fn parse_variable(tokens: &mut Tokens) -> Node {
    let name = tokens.parse_identifer();
    tokens.expect_token(Token::Equals);
    let value = parse_expression(tokens, true).unwrap();
    Node::Variable { name, value }
}
