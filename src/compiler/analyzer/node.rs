use crate::compiler::types::{BaseType, Type};

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(String, IRType)>,
    pub return_type: IRType,
    pub operations: Vec<Operation>,
}

#[derive(Debug)]
pub enum Operation {
    Label(String),
    Allocate(String, IRType),
    Store(IRType, IRValue, String),
    Load(String, IRType, String),
    Return(IRType, IRValue),
}

#[derive(Debug)]
pub enum IRValue {
    IntLiteral(String),
    FloatLiteral(String),
    Variable(String),
    Null,
}
impl std::fmt::Display for IRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IntLiteral(int) => format!("{}", int),
                Self::Variable(key) => format!("%{}", key),
                _ => todo!("{:#?}", self)
            }
        )
    }
}


#[derive(Debug)]
pub enum IRType {
    Tuple(Vec<IRType>),
    Pointer(Box<IRType>),
    Integer(usize),
    UInteger(usize),
    Array(Box<IRType>, usize),
    Struct(String),
    Float,
    Double,
    Void,
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
                Self::Array(t, size) => format!("[ {} x {} ]", size, t),
                Self::Integer(bits) | IRType::UInteger(bits) => format!("i{}", bits),
                Self::Pointer(t) => format!("*{}", t),
                Self::Struct(name) => format!("%{}", name),
                Self::Tuple(types) => format!("{{ {} }}", {
                    let mut strings = Vec::new();
                    for t in types {
                        strings.push(format!("{}", t))
                    }
                    strings.join(", ")
                }),
            }
        )
    }
}
impl Type {
    pub fn convert(&self) -> IRType {
        match self {
            Type::Base(base) => match base {
                BaseType::Boolean => IRType::Integer(1),
                BaseType::Float32 => IRType::Float,
                BaseType::Float64 => IRType::Double,
                BaseType::Void => IRType::Void,
                BaseType::Never => IRType::Void,
                BaseType::Int8 => IRType::Integer(8),
                BaseType::Int16 => IRType::Integer(16),
                BaseType::Int32 => IRType::Integer(32),
                BaseType::Int64 => IRType::Integer(64),

                BaseType::UInt8 => IRType::UInteger(8),
                BaseType::UInt16 => IRType::UInteger(16),
                BaseType::UInt32 => IRType::UInteger(32),
                BaseType::UInt64 => IRType::UInteger(64),
                BaseType::StaticString => todo!(),
            },
            Type::Struct(name) => IRType::Struct(name.clone()),
            Type::Pointer(dt) | Type::Reference(dt) => IRType::Pointer(Box::new(dt.convert())),
            Type::Array(dt, size) => IRType::Array(Box::new(dt.convert()), size.clone()),
            Type::Tuple(dts) => {
                if dts.len() == 0 {
                    return IRType::Void;
                }
                return IRType::Tuple(
                    dts.into_iter()
                        .map(|t| t.convert())
                        .collect::<Vec<IRType>>(),
                );
            }
            Type::Unkown => panic!(),
        }
    }
}
