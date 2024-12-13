use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{
    expression::parse_expression, identifier::parse_after_identifier, ifstatement::parse_ifstatement, loops::{parse_loop, parse_while}, namespace::parse_namespace, variable::parse_variable
};

pub fn parse_body(tokens: &mut Tokens) -> CompileResult<Vec<NodeInfo>> {
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
                Token::Loop,
                Token::While,
                Token::Continue,
                Token::Break,
                Token::Identifier(String::new()),
            ],
            true,
        )?;

        let node = match info.token {
            Token::StartScope => {
                let nodes = parse_body(tokens)?;
                tokens.create_node(Node::Scope(nodes))
            },
            Token::Continue => tokens.create_node(Node::Continue),
            Token::Break => tokens.create_node(Node::Break),
            Token::If => parse_ifstatement(tokens)?,
            Token::Use => parse_namespace(tokens, false)?,
            Token::Identifier(name) => parse_after_identifier(tokens, name)?,
            Token::Loop => parse_loop(tokens)?,
            Token::While => parse_while(tokens)?,
            Token::Return => {
                let expression = parse_expression(tokens, false)?;
                tokens.create_node(Node::Return(expression))
            }
            Token::Variable => parse_variable(tokens)?,
            _ => continue,
        };
        body.push(node)
    }

    return Ok(body);
}
