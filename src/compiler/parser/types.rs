use crate::{
    compiler::nodes::ast::{RawType, Type},
    diagnostics::DiagnosticResult,
};

use super::Parser;

impl Parser {
    pub fn parse_type(&mut self) -> DiagnosticResult<Type> {
        let string = self.expect_identifier()?;
        let raw = match string.raw.as_str() {
            "i32" => RawType::Int(32),
            _ => todo!(),
        };
        Ok(Type::new(raw, string.position))
    }
}
