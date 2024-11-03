use crate::compiler::{
    lexer::{Token, Tokens},
    parser::Node,
    types::{BaseType, Type},
};

use super::{body::parse_body, types::parse_type};

pub fn parse_function(tokens: &mut Tokens) -> Node {
    let name = tokens.parse_identifer();
    tokens.expect_token(Token::OpenParen);

    let mut parameters: Vec<(String, Type)> = Vec::new();
    loop {
        if tokens.peek_expect_token(Token::CloseParen) {
            tokens.advance();
            break;
        }
        let name = tokens.parse_identifer();
        let data_type= parse_type(tokens);
        parameters.push((name, data_type))
    }

    let return_type = if tokens.peek_expect_token(Token::Colon) {
        tokens.advance();
        parse_type(tokens)
    } else {
        Type::Base(BaseType::Void)
    };

    tokens.expect_token(Token::StartScope);
    let body = parse_body(tokens);

    Node::Function { name, parameters, return_type, body }
}
