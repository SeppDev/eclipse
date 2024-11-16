use crate::compiler::{
    errors::MessageKind,
    lexer::{Token, Tokens},
    types::{BaseType, Type},
};

pub fn parse_type(tokens: &mut Tokens) -> Type {
    if tokens
        .peek_expect_tokens(vec![Token::OpenParen], true)
        .is_some()
    {
        let mut tuple = Vec::new();
        loop {
            let new_type = parse_type(tokens);
            tuple.push(new_type);
            match tokens
                .expect_tokens(vec![Token::CloseParen, Token::Comma], false)
                .token
            {
                Token::CloseParen => break,
                Token::Comma => continue,
                _ => panic!(),
            };
        }
        return Type::Tuple(tuple);
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
        Token::Ampersand => return Type::Reference(Box::new(parse_type(tokens))),
        Token::Asterisk => return Type::Pointer(Box::new(parse_type(tokens))),
        Token::Identifier(string) => string,
        t => return Type::Unkown,
    };

    return match name.as_str() {
        "i64" => Type::Base(BaseType::Int64),
        "u64" => Type::Base(BaseType::UInt64),
        "i32" => Type::Base(BaseType::Int32),
        "u32" => Type::Base(BaseType::UInt32),
        "i16" => Type::Base(BaseType::Int16),
        "u16" => Type::Base(BaseType::UInt16),
        "i8" => Type::Base(BaseType::Int8),
        "u8" => Type::Base(BaseType::UInt8),
        // "f16" => Type::Base(BaseType::Float16),
        "f32" => Type::Base(BaseType::Float32),
        "f64" => Type::Base(BaseType::Float64),
        // "f128" => Type::Base(BaseType::Float128),
        "bool" => Type::Base(BaseType::Boolean),
        str => {
            tokens.throw(
                MessageKind::Error,
                info.location.clone(),
                format!("Expected type, got {}", str),
                "",
            );
            Type::Unkown
        }
    };
}
