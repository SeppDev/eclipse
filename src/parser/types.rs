use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{node::Type, tokens_expected_got, BaseType};

pub fn parse_type(tokens: &mut TokensGroup) -> Result<Type, BuildError> {
    let name = match tokens.advance() {
        Ok(info) => match info.token {
            Token::OpenParen => {
                let mut types = Vec::new();

                loop {
                    let info = tokens.peek()?; 
                    match info.token {
                        Token::CloseParen => {
                            tokens.advance()?;
                            break
                        },
                        _ => {}
                    }

                    types.push(match parse_type(tokens) {
                        Ok(t) => t,
                        Err(error) => return Err(error),
                    });

                    let info = tokens.advance()?;
                    match info.token {
                        Token::CloseParen => break,
                        Token::Comma => continue,
                        _ => {
                            return Err(tokens_expected_got(
                                tokens,
                                vec![Token::CloseParen, Token::Comma],
                                info,
                            ))
                        }
                    }
                }

                return Ok(Type::Tuple(types));
            }
            Token::Identifier(name) => name,
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("Type"))],
                    info,
                ))
            }
        },
        Err(error) => return Err(error),
    };

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
