use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{
    expression::parse_expression, identifier::parse_after_identifier, namespace::parse_namespace,
    variable::parse_variable,
};

pub fn parse_body(tokens: &mut Tokens) -> Vec<NodeInfo> {
    let mut body: Vec<NodeInfo> = Vec::new();

    loop {
        if tokens
            .peek_expect_tokens(vec![Token::EndScope], false)
            .is_some()
        {
            break;
        }

        let info = tokens.expect_tokens(
            vec![
                Token::Return,
                Token::Function,
                Token::Variable,
                Token::StartScope,
                Token::Use,
                Token::Identifier(String::new()),
            ],
            true,
        );

        let node = match info.token {
            Token::StartScope => {
                let nodes = parse_body(tokens);
                tokens.expect_tokens(vec![Token::EndScope], false);
                tokens.create_node(Node::Scope(nodes))
            }
            Token::Use => parse_namespace(tokens, false),
            Token::Identifier(name) => parse_after_identifier(tokens, name),
            Token::Return => {
                let expression = parse_expression(tokens, false);
                tokens.create_node(Node::Return(expression))
            }
            Token::Variable => parse_variable(tokens),
            _ => continue
        };
        body.push(node)
    }

    return body;
}
