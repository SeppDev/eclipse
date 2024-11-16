use crate::compiler::{
    counter::NameCounter,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
    types::{BaseType, Type},
};

use super::{body::parse_body, types::parse_type};

pub fn parse_function(tokens: &mut Tokens, public: bool) -> NodeInfo {
    let name = tokens.parse_identifier().unwrap();
    tokens.expect_tokens(vec![Token::OpenParen], false);

    let mut parameters: Vec<(String, Type)> = Vec::new();
    loop {
        if tokens
            .peek_expect_tokens(vec![Token::CloseParen], true)
            .is_some()
        {
            break;
        }
        let name = match tokens.parse_identifier() {
            Some(s) => s,
            None => break,
        };
        let data_type = parse_type(tokens);
        parameters.push((name, data_type));

        match tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false).token {
            Token::CloseParen => break,
            Token::Comma => continue,
            _ => break
        }
    }

    let return_type = if tokens
        .peek_expect_tokens(vec![Token::Colon], true)
        .is_some()
    {
        parse_type(tokens)
    } else {
        Type::Base(BaseType::Void)
    };

    tokens.expect_tokens(vec![Token::StartScope], false);
    let body = parse_body(tokens);

    tokens.create_node(Node::Function {
        public,
        name,
        parameters,
        return_type,
        body,
    })
}
