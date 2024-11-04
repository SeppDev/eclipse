use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{
    expression::parse_expression, identifier::parse_after_identifier, variable::parse_variable,
};

pub fn parse_body(tokens: &mut Tokens) -> Vec<NodeInfo> {
    let mut body: Vec<NodeInfo> = Vec::new();

    loop {
        if tokens
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_some()
        {
            break;
        }

        let info = tokens.expect_tokens(
            vec![
                Token::Return,
                Token::Function,
                Token::Variable,
                Token::Identifier(String::new()),
            ],
            true,
        );

        let node = match info.token {
            Token::Identifier(name) => parse_after_identifier(tokens, name),
            Token::Return => {
                let expression = parse_expression(tokens, false);
                tokens.create_node(Node::Return(expression))
            }
            Token::Variable => parse_variable(tokens),
            t => tokens.throw_error(format!("Expected item, got '{}'", t), ""),
        };
        body.push(node)
    }

    return body;
}
