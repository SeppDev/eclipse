use eclipse::ParseError;

use super::{token_expected, tokens::parse_tokens, Node};
use crate::parser::{lexer::Token, tokens_group::TokensGroup};

pub fn scope(group: &mut TokensGroup) -> Result<Vec<Node>, ParseError> {
    let nodes = parse_tokens(group);

    match group.next_token() {
        Some(info) => match info.token {
            Token::EndScope => {}
            _ => return Err(token_expected(Token::EndScope, info)),
        },
        None => return Err(ParseError::NoTokenFound),
    };

    return nodes;
}
