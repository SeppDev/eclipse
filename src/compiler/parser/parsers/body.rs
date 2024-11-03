use crate::compiler::{lexer::{Token, Tokens}, parser::Node};

use super::{expression::parse_expression, variable::parse_variable};


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
            Token::EndScope => break,
            t => tokens.throw_error(format!("Expected item, got '{}'", t), "")
        };
        body.push(node)
    }

    return body
}
