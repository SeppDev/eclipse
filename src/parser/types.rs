use crate::{lexer::TokensGroup, CompileError};

use super::{node::Type, parser::get_identifier, BaseType};

pub fn parse_type(tokens: &mut TokensGroup) -> Result<Type, CompileError> {
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
