use crate::compiler::{
    counter::NameCounter,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
    types::{BaseType, Type},
};

use super::{body::parse_body, expression::parse_expression};
pub fn parse_ifstatement(tokens: &mut Tokens) -> NodeInfo {
    let expression = parse_expression(tokens, true).unwrap();
    tokens.expect_tokens(vec![Token::StartScope], false);

    let body = parse_body(tokens);

    let else_expression = if tokens.peek_expect_tokens(vec![Token::Else], true).is_some() {
        tokens.expect_tokens(vec![Token::StartScope], false);
        let body = parse_body(tokens);
        Some(body)
    } else {
        None
    };

    tokens.create_node(Node::IfStatement {
        expression: (expression, body),
        elseif: Vec::new(),
        else_expression: else_expression,
    })
}
