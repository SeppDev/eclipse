use eclipse::ParseError;

use super::{token_expected, Node};
use crate::parser::{lexer::Token, tokens_group::TokensGroup};

pub fn module(group: &mut TokensGroup) -> Result<Node, ParseError> {
    let module = match group.next_token() {
        Some(info) => match info.token {
            Token::Identifier(name) => name,
            Token::String(string) => string,
            _ => return Err(token_expected(Token::Identifier("module".to_string()), info))
        },
        None => return Err(ParseError::NoTokenFound)
    };
    return Ok(Node::Module(module))
}
