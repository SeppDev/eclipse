use crate::compiler::{
    lexer::{Token, Tokens},
    parser::Node,
};

use super::{expression::parse_expression, types::parse_type};

pub fn parse_variable(tokens: &mut Tokens) -> Node {
    let mutable = tokens.peek_expect_token(Token::Mutable);
    let name = tokens.parse_identifer();

    let data_type = if tokens.peek_expect_token(Token::Colon) {
        Some(parse_type(tokens))
    } else {
        None
    };

    tokens.expect_token(Token::Equals);
    let expression = parse_expression(tokens, true).unwrap();

    tokens.create_node(Node::Variable {
        name,
        mutable,
        data_type,
        expression,
    })
}

pub fn parse_set_variable(tokens: &mut Tokens, name: String) -> Node {
    let expression = parse_expression(tokens, true).unwrap();
    tokens.create_node(Node::SetVariable { name, expression })
}
