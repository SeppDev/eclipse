use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    types::{BaseType, Type},
};

pub fn parse_type(tokens: &mut Tokens) -> CompileResult<Type> {
    if tokens
        .peek_expect_tokens(vec![Token::OpenParen], true)
        .is_some()
    {
        let mut tuple = Vec::new();
        loop {
            let new_type = parse_type(tokens)?;
            tuple.push(new_type);

            let result = tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false);
            match result.token {
                Token::CloseParen => break,
                Token::Comma => continue,
                _ => panic!(),
            };
        }
        return Ok(Type::Tuple(tuple));
    }

    let info = tokens.expect_tokens(
        vec![
            Token::Ampersand,
            Token::Asterisk,
            Token::Identifier(String::new()),
        ],
        false,
    );

    let name = match info.token {
        Token::Ampersand => return Ok(Type::Reference(Box::new(parse_type(tokens)?))),
        Token::Asterisk => return Ok(Type::Pointer(Box::new(parse_type(tokens)?))),
        Token::Identifier(string) => string,
        _ => return Ok(Type::Unkown),
    };

    let t = match name.as_str() {
        "i64" => Type::Base(BaseType::Int64),
        "u64" => Type::Base(BaseType::UInt64),
        "i32" => Type::Base(BaseType::Int32),
        "u32" => Type::Base(BaseType::UInt32),
        "i16" => Type::Base(BaseType::Int16),
        "u16" => Type::Base(BaseType::UInt16),
        "i8" => Type::Base(BaseType::Int8),
        "u8" => Type::Base(BaseType::UInt8),
        "f32" => Type::Base(BaseType::Float32),
        "f64" => Type::Base(BaseType::Float64),
        "bool" => Type::Base(BaseType::Boolean),
        "!" => Type::Base(BaseType::Never),
        str => {
            tokens.error(info.location.clone(), format!("Expected type, got {}", str));
            Type::Unkown
        }
    };
    return Ok(t);
}
