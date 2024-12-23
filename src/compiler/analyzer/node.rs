use crate::compiler::{types::{BaseType, ReferenceState, Type}, POINTER_WITH};

#[derive(Debug)]
pub enum IRValue {
    BoolLiteral(bool),
    IntLiteral(String),
    FloatLiteral(String),
    Variable(String),
    Arguments {
        return_pointers: Vec<(IRType, String)>,
        arguments: Vec<(IRType, IRValue)>,
    },
    Null,
}

fn display_arguments(
    return_pointers: &Vec<(IRType, String)>,
    arguments: &Vec<(IRType, IRValue)>,
) -> String {
    let mut args = return_pointers
        .iter()
        .map(|(data_type, destination)| format!("ptr sret({data_type}) %{destination}"))
        .collect::<Vec<String>>();

    args.extend(
        arguments
            .iter()
            .map(|(data_type, value)| format!("{data_type} {value}"))
            .collect::<Vec<String>>(),
    );

    return args.join(", ");
}

impl std::fmt::Display for IRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BoolLiteral(bool) => format!("{}", if bool == &true { 1 } else { 0 }),
                Self::IntLiteral(value) | Self::FloatLiteral(value) => format!("{value}"),
                Self::Variable(key) => format!("%{key}"),
                // Self::StringLiteral(str) => format!("\"{str}\\00\""),
                Self::Arguments {
                    return_pointers,
                    arguments,
                } => format!("{}", display_arguments(return_pointers, arguments)),
                Self::Null => panic!("Whoops, null found!"),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum IRType {
    Tuple(Vec<IRType>),
    PointerType(Box<IRType>),
    Integer(usize),
    Array(usize, Box<IRType>),
    Bytes(usize),
    Pointer,
    Float,
    Double,
    Void,
}
impl IRType {
    fn pointer(self) -> IRType {
        return IRType::PointerType(Box::new(self));
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
                Self::Bytes(bytes) => format!("[{bytes} x i8]"),
                Self::Integer(bits) => format!("i{bits}"),
                Self::PointerType(t) => format!("{t}*"),
                Self::Pointer => format!("ptr"),
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
            BaseType::Void | BaseType::Never => IRType::Void,
            BaseType::Usize | BaseType::ISize => IRType::Integer(POINTER_WITH),
            
            BaseType::Float32 => IRType::Float,
            BaseType::Float64 => IRType::Double,

            BaseType::Boolean => IRType::Integer(1),
            BaseType::Int(bits) | BaseType::UInt(bits) => IRType::Integer(bits.clone()),

            BaseType::StaticString => todo!(), //IRType::Array(size.clone(), Box::new(IRType::Integer(8))),

            BaseType::Array(size, t) => IRType::Bytes(*size * t.bytes()),
            // BaseType::Array(size, t) => IRType::Array(*size, Box::new(t.convert())),
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
