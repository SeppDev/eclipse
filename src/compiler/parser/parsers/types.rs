use crate::compiler::{
    lexer::Tokens,
    types::{BaseType, Type},
};

pub fn parse_type(tokens: &mut Tokens) -> Type {
    let name = tokens.parse_identifer();

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
        str => tokens.throw_error(format!("Expected type, got {}", str), ""),
    };
}
