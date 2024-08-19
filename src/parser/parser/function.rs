use eclipse::ParseError;

use super::{parse_type::parse_type, scope::scope, token_expected, Node};
use crate::parser::{lexer::Token, node::Type, tokens_group::TokensGroup};

pub fn function(group: &mut TokensGroup) -> Result<Node, ParseError> {
    let name = match group.next_token() {
        Some(tokeninfo) => match tokeninfo.token.clone() {
            Token::Identifier(name) => name,
            _ => {
                return Err(token_expected(
                    Token::Identifier(format!("function")),
                    tokeninfo,
                ))
            }
        },
        None => return Err(ParseError::NoTokenFound),
    };

    let tokeninfo = match group.next_token() {
        Some(tokeninfo) => tokeninfo,
        None => return Err(ParseError::NoTokenFound),
    };

    match tokeninfo.token {
        Token::OpenParen => {}
        _ => return Err(token_expected(Token::OpenParen, tokeninfo)),
    }

    let mut parameters = Vec::new();
    loop {
        let tokeninfo = match group.next_token() {
            Some(tokeninfo) => tokeninfo,
            None => return Err(ParseError::NoTokenFound),
        };
        let name = match tokeninfo.token {
            Token::CloseParen => break,
            Token::Identifier(name) => name,
            _ => {
                return Err(token_expected(
                    Token::Identifier(format!("parameter")),
                    tokeninfo,
                ))
            }
        };

        let tokeninfo = match group.next_token() {
            Some(tokeninfo) => tokeninfo,
            None => return Err(ParseError::NoTokenFound),
        };
        match tokeninfo.token {
            Token::Colon => {}
            _ => return Err(token_expected(Token::Colon, tokeninfo)),
        }

        let t = match parse_type(group) {
            Ok(t) => t,
            Err(error) => return Err(error),
        };
        parameters.push((name, t));

        let tokeninfo = match group.peek() {
            Some(tokeninfo) => tokeninfo,
            None => return Err(ParseError::NoTokenFound),
        };
        match tokeninfo.token {
            Token::Comma => {
                group.next_token();
            }
            Token::CloseParen => {
                group.next_token();
                break;
            }
            _ => return Err(token_expected(Token::Comma, tokeninfo)),
        }
    }

    let tokeninfo = match group.peek() {
        Some(tokeninfo) => tokeninfo,
        None => return Err(ParseError::NoTokenFound),
    };

    let return_type: Option<Type> = match tokeninfo.token.clone() {
        Token::StartScope => None,
        Token::Colon => {
            group.next_token();
            match parse_type(group) {
                Ok(t) => Some(t),
                Err(error) => return Err(error),
            }
        }
        _ => return Err(token_expected(Token::Colon, tokeninfo)),
    };

    let body = match group.next_token() {
        Some(tokeninfo) => {
            let scope = match tokeninfo.token {
                Token::StartScope => scope(group),
                _ => return Err(token_expected(Token::StartScope, tokeninfo)),
            };
            match scope {
                Ok(nodes) => nodes,
                Err(error) => return Err(error),
            }
        }
        None => return Err(ParseError::NoTokenFound),
    };

    let function = Node::Function {
        name: name,
        parameters,
        return_type: return_type,
        body: body,
    };

    return Ok(function);
}
