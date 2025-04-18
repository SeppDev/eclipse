use crate::{
    compiler::nodes::ast::{RawType, Type},
    diagnostics::DiagnosticResult,
};

use super::Parser;

impl Parser {
    pub fn parse_type(&mut self) -> DiagnosticResult<Type> {
        let info = self.expect_identifier()?;
        let raw = match info.string.as_str() {
            "i64" => RawType::Int(64),
            "i32" => RawType::Int(32),
            "i16" => RawType::Int(16),
            "i8" => RawType::Int(8),

            "u64" => RawType::Int(64),
            "u32" => RawType::UInt(32),
            "u16" => RawType::UInt(16),
            "u8" => RawType::UInt(8),

            "bool" => RawType::Boolean,
            _ => todo!(),
        };
        Ok(Type::new(raw, info.position))
    }
}
