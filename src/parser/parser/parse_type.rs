use eclipse::ParseError;

use crate::parser::{
    lexer::Token,
    node::{Integer, Type},
    tokens_group::TokensGroup,
};

use super::token_expected;

pub fn parse_type(group: &mut TokensGroup) -> Result<Type, ParseError> {
    let tokeninfo = match group.next_token() {
        Some(tokeninfo) => tokeninfo,
        None => return Err(ParseError::NoTokenFound),
    };

    let string = match tokeninfo.token {
        Token::Identifier(string) => string,
        _ => {
            return Err(token_expected(
                Token::Identifier(format!("type")),
                tokeninfo,
            ))
        }
    };

    let t = match string.as_str() {
        "i64" => Type::Integer(Integer::i64),
        // "u64" => Type::Integer(Integer::u64),
        // "i32" => Type::Integer(Integer::i32),
        // "u32" => Type::Integer(Integer::u32),
        // "i16" => Type::Integer(Integer::i16),
        // "u16" => Type::Integer(Integer::u16),
        // "i8" => Type::Integer(Integer::i8),
        // "u8" => Type::Integer(Integer::u8),
        "String" => Type::String,
        "bool" => Type::Boolean,
        _ => return Err(ParseError::Type),
    };

    return Ok(t);
}
