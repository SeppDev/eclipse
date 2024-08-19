use eclipse::ParseError;

use crate::parser::{lexer::Token, node::Node, tokens_group::TokensGroup};

use super::{result::result, scope::scope, token_expected};

pub fn conditional(group: &mut TokensGroup) -> Result<Node, ParseError> {
    match group.next_token().unwrap().token {
        Token::OpenParen => {}
        token => panic!("Expected '(' got, {:?}", token),
    }

    let a = match result(group) {
        Ok(a) => match a {
            Some(a) => a,
            None => return Err(ParseError::NoTokenFound)
        },
        Err(b) => return Err(b),
    };
    match group.next_token().unwrap().token {
        Token::Compare => {}
        token => panic!("Expected '==' got, {:?}", token),
    }
    let b = match result(group) {
        Ok(a) => match a {
            Some(a) => a,
            None => return Err(ParseError::NoTokenFound)
        },
        Err(b) => return Err(b),
    };
    match group.next_token().unwrap().token {
        Token::CloseParen => {}
        token => panic!("Expected ')' got, {:?}", token),
    }

    match group.next_token().unwrap().token {
        Token::StartScope => {}
        _ => panic!(),
    }

    let body = match scope(group) {
        Ok(a) => a,
        Err(b) => return Err(b),
    };
    let else_body: Option<Vec<Node>> = match group.peek() {
        Some(tokeninfo) => match tokeninfo.token {
            Token::StartScope => match scope(group) {
                Ok(a) => Some(a),
                Err(error) => return Err(error),
            },
            _ => return Err(token_expected(Token::StartScope, tokeninfo)),
        },
        None => None,
    };

    Ok(Node::Conditional((a, b), body, else_body))
}
