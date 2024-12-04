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
        return Ok(Type::new(BaseType::Tuple(tuple)));
    }

    let info = tokens.expect_tokens(
        vec![
            Token::Ampersand,
            Token::Asterisk,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::Identifier(String::new()),
        ],
        false,
    );

    let name = match info.token {
        Token::Ampersand => return Ok(parse_type(tokens)?.to_reference()?),
        Token::Asterisk => return Ok(parse_type(tokens)?.to_pointer()?),
        Token::OpenBracket => {
            let data_type = parse_type(tokens)?;
            let _ = tokens.expect_tokens(vec![Token::SemiColon], false);
            let info = tokens.expect_tokens(vec![Token::Integer(String::new())], false);
            let count = match info.token {
                Token::Integer(count) => count.parse::<usize>().unwrap(),
                _ => panic!()
            };

            let _ = tokens.expect_tokens(vec![Token::CloseBracket], false);
            return Ok(Type::new(BaseType::Array(count, Box::new(data_type))))
        },
        Token::Identifier(string) => string,
        _ => return Ok(Type::void()),
    };

    let t = match name.as_str() {
        "i64" => Type::new(BaseType::Int(64)),
        "u64" => Type::new(BaseType::UInt(64)),
        "i32" => Type::new(BaseType::Int(32)),
        "u32" => Type::new(BaseType::UInt(32)),
        "i16" => Type::new(BaseType::Int(16)),
        "u16" => Type::new(BaseType::UInt(16)),
        "i8" => Type::new(BaseType::Int(8)),
        "u8" => Type::new(BaseType::UInt(8)),
        "f32" => Type::new(BaseType::Float32),
        "f64" => Type::new(BaseType::Float64),
        "bool" => Type::new(BaseType::Boolean),
        "!" => Type::new(BaseType::Never),
        str => {
            tokens.error(info.location.clone(), format!("Expected type, got {}", str));
            Type::void()
        }
    };
    return Ok(t);
}
