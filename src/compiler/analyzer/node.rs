use crate::compiler::types::{BaseType, ReferenceState, Type};


#[derive(Debug, Clone)]
pub enum IRValue {
    BoolLiteral(bool),
    IntLiteral(String),
    FloatLiteral(String),
    Variable(String),
    Arguments(Vec<(IRType, IRValue)>),
    Null,
}
impl std::fmt::Display for IRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BoolLiteral(bool) => format!("{}", if bool == &true { 1 } else { 0 }),
                Self::IntLiteral(int) => format!("{int}"),
                Self::FloatLiteral(float) => format!("{float}"),
                Self::Variable(key) => format!("%{key}"),
                // Self::StringLiteral(str) => format!("\"{str}\\00\""),
                Self::Arguments(arguments) => arguments
                    .iter()
                    .map(|(data_type, value)| format!("{data_type} {value}"))
                    .collect::<Vec<String>>()
                    .join(", "),
                Self::Null => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum IRType {
    Tuple(Vec<IRType>),
    Pointer(Box<IRType>),
    Integer(usize),
    Array(usize, Box<IRType>),
    // Bytes(usize),
    Float,
    Double,
    Void,
}
impl IRType {
    fn pointer(self) -> IRType {
        return IRType::Pointer(Box::new(self));
    }
    pub fn signed(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            // Self::UInteger(_) => false,
            _ => panic!("{}", self),
        }
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float | Self::Double)
    }
    // pub fn is_void(&self) -> bool {
    //     matches!(self, Self::Void)
    // }
}

impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => "void".to_string(),
                Self::Double => "double".to_string(),
                Self::Float => "float".to_string(),
                Self::Array(size, t) => format!("[{size} x {t}]"),
                // Self::Bytes(bytes) => format!("[{bytes} x i8]"),
                Self::Integer(bits) => format!("i{bits}"),
                Self::Pointer(t) => format!("{t}*"),
                // Self::Struct(name) => format!("%{name}",),
                Self::Tuple(types) => format!("{{ {} }}", {
                    types
                        .iter()
                        .map(|value| format!("{value}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                }),
            }
        )
    }
}
impl Type {
    pub fn convert(&self) -> IRType {
        let mut ir = match &self.base {
            BaseType::Void => IRType::Void,
            BaseType::Never => IRType::Void,

            BaseType::Float32 => IRType::Float,
            BaseType::Float64 => IRType::Double,

            BaseType::Boolean => IRType::Integer(1),
            BaseType::Int(bits) | BaseType::UInt(bits) => IRType::Integer(bits.clone()),

            BaseType::StaticString(_size) => todo!(), //IRType::Array(size.clone(), Box::new(IRType::Integer(8))),

            BaseType::Array(size, t) => IRType::Array(*size, Box::new(t.convert())), //IRType::Bytes(*size * t.bytes()),
            BaseType::Tuple(dts) => {
                if dts.len() == 0 {
                    return IRType::Void;
                } else if dts.len() == 1 {
                    return dts.clone().pop().unwrap().convert();
                }

                return IRType::Tuple(
                    dts.into_iter()
                        .map(|t| t.convert())
                        .collect::<Vec<IRType>>(),
                );
            }
        };
        let count = match self.ref_state {
            ReferenceState::Pointer(p) => p,
            ReferenceState::Shared | ReferenceState::Mutable => 1,
            ReferenceState::None => 0,
        };

        for _ in 0..count {
            ir = ir.pointer()
        }

        return ir;
    }
}
