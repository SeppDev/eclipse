use crate::compiler::{
    counter::NameCounter, lexer::{Token, Tokens}, parser::{Node, NodeInfo}
};

use super::{
    expression::parse_expression, identifier::parse_after_identifier, ifstatement::parse_ifstatement, namespace::parse_namespace, variable::parse_variable
};

pub fn parse_body(name_counter: &mut NameCounter, tokens: &mut Tokens) -> Vec<NodeInfo> {
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
                Token::StartScope,
                Token::If,
                Token::Use,
                Token::Identifier(String::new()),
            ],
            true,
        );

        let node = match info.token {
            Token::StartScope => {
                let nodes = parse_body(name_counter, tokens);
                // tokens.expect_tokens(vec![Token::EndScope], false);
                tokens.create_node(Node::Scope(nodes))
            }
            Token::If => parse_ifstatement(name_counter, tokens),
            Token::Use => parse_namespace(tokens, false),
            Token::Identifier(name) => parse_after_identifier(tokens, name),
            Token::Return => {
                let expression = parse_expression(tokens, false);
                tokens.create_node(Node::Return(expression))
            }
            Token::Variable => parse_variable(tokens),
            _ => continue,
        };
        body.push(node)
    }

    return body;
}
