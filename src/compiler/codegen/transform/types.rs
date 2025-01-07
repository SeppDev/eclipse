use crate::compiler::nodes::{hlir, ir};

impl hlir::Type {
    pub fn convert(&self) -> ir::Type {
        use ir::Type;
        match &self {
            Self::Void | Self::Never => Type::Void,
            Self::Int(bits) | Self::UInt(bits) => Type::Int(*bits),
            Self::Boolean => Type::Int(1),
            Self::Array(size, dt) => Type::Array(*size, Box::new(dt.convert())),
            Self::Float32 => Type::Float32,
            Self::Float64 => Type::Float64,
            Self::Bytes(bytes) => Type::Bytes(*bytes),
            Self::Reference(r) => Type::Pointer(Box::new(r.convert())),
            Self::Tuple(f) => Type::Tuple(f.into_iter().map(|f| f.convert()).collect()),
        }
    }
}
