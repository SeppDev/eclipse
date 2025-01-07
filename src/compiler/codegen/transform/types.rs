use crate::compiler::{
    codegen::target::Target,
    nodes::{hlir, ir},
};

impl Target {
    pub(super) fn convert(&self, data_type: &hlir::Type) -> ir::Type {
        use ir::Type;
        match &data_type {
            hlir::Type::Void | hlir::Type::Never => Type::Void,
            hlir::Type::Usize | hlir::Type::Isize => Type::Int(self.pointer_width()),
            hlir::Type::Float32 => Type::Float32,
            hlir::Type::Float64 => Type::Float64,

            hlir::Type::Boolean => Type::Int(1),
            hlir::Type::Array(size, dt) => Type::Array(*size, Box::new(self.convert(dt))),
            hlir::Type::Bytes(bytes) => Type::Bytes(*bytes),
            hlir::Type::Reference(dt) => Type::Pointer(Box::new(self.convert(dt))),
            hlir::Type::Tuple(f) => Type::Tuple(f.into_iter().map(|dt| self.convert(dt)).collect()),
            hlir::Type::Int(bits) | hlir::Type::UInt(bits) => Type::Int(*bits),
        }
    }
}
