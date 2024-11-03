use crate::compiler::{lexer::{Token, Tokens}, parser::Node};

use super::{expression::parse_expression, identifier::parse_after_identifier, variable::{parse_set_variable, parse_variable}};


pub fn parse_body(tokens: &mut Tokens) -> Vec<Node> {
    let mut body: Vec<Node> = Vec::new();

    loop {
        let info = tokens.start();
        let node = match info.token.clone() {
            Token::Return => {
                let expression = parse_expression(tokens, false);
                Node::Return(expression)
            },
            Token::Variable => parse_variable(tokens),
            Token::Identifier(name) => parse_after_identifier(tokens, name),
            Token::EndScope => break,
            t => tokens.throw_error(format!("Expected item 2, got '{}'", t), "")
        };
        body.push(node)
    }

    return body
}
