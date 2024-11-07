use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
    types::{BaseType, Type},
};

use super::{body::parse_body, types::parse_type};

pub fn parse_function(tokens: &mut Tokens) -> NodeInfo {
    let name = tokens.parse_identifer();
    tokens.expect_tokens(vec![Token::OpenParen], false);

    let mut parameters: Vec<(String, Type)> = Vec::new();
    loop {
        if tokens.peek_expect_token(Token::CloseParen, true) {
            break;
        }
        let name = tokens.parse_identifer();
        let data_type = parse_type(tokens);
        parameters.push((name, data_type))
    }

    let return_type = if tokens.peek_expect_token(Token::Colon, true) {
        parse_type(tokens)
    } else {
        Type::Base(BaseType::Void)
    };

    tokens.expect_tokens(vec![Token::StartScope], false);
    let body = parse_body(tokens);
     tokens.expect_tokens(vec![Token::EndScope], false);

    tokens.create_node(Node::Function {
        name,
        parameters,
        return_type,
        body,
    })
}
