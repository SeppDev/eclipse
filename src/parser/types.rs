use crate::{
    lexer::{Token, TokensGroup},
    ParseResult,
};

use super::{node::Type, parser::get_identifier, peek_expect_tokens, BaseType};

pub fn parse_type(tokens: &mut TokensGroup) -> ParseResult<Type> {
    let info = peek_expect_tokens(tokens, vec![Token::OpenParen], true)?;
    if info.is_some() {
        let mut tuple = Vec::new();
        loop {
            let info = peek_expect_tokens(tokens, vec![Token::Comma, Token::CloseParen], true)?;
            let info = match info {
                Some(info) => info,
                None => {
                    let t = parse_type(tokens)?;
                    tuple.push(t);
                    continue;
                }
            };

            match info.token {
                Token::CloseParen => break,
                _ => continue,
            }
        }

        return Ok(Type::Tuple(tuple));
    }

    let name = get_identifier(tokens)?;

    return Ok(match name.as_str() {
        "i64" => Type::Base(BaseType::Int64),
        "u64" => Type::Base(BaseType::UInt64),
        "i32" => Type::Base(BaseType::Int32),
        "u32" => Type::Base(BaseType::UInt32),
        "i16" => Type::Base(BaseType::Int16),
        "u16" => Type::Base(BaseType::UInt16),
        "i8" => Type::Base(BaseType::Int8),
        "u8" => Type::Base(BaseType::UInt8),
        "f64" => Type::Base(BaseType::Float64),
        "f32" => Type::Base(BaseType::Float32),
        "bool" => Type::Base(BaseType::Boolean),
        a => Type::Custom(a.to_string()),
    });
}
