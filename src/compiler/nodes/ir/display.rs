use super::{BinaryOperation, BinaryOperationPrefix, Type, Value};
use std::fmt::Display;

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Add(prefix) => format!("{prefix}add"),
            Self::Subtract(prefix) => format!("{prefix}sub"),
            Self::Divide(prefix) => format!("{prefix}div"),
            Self::Multiply(prefix) => format!("{prefix}mul"),
            Self::Remainder(prefix) => format!("{prefix}rem"),

        })
    }
}
impl Display for BinaryOperationPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Float => format!("f"),
                Self::Unsigned => format!("u"),
                Self::Signed => format!(""),
            }
        )
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Reference(body) | Self::Register(body) => format!("%{body}"),
                Self::Integer(body) => format!("{body}"),
                Self::Float(body) => format!("{body}"),
                Self::Boolean(body) => format!("{body}"),
            }
        )
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => format!("void"),
                Self::Bytes(bytes) => format!("[{bytes} x i8]"),
                Self::Array(size, data_type) => format!("[{size} x {data_type}]"),
                Self::Boolean => format!("i1"),
                Self::Int(bits) => format!("i{bits}"),
                Self::Pointer(data_type) => format!("{data_type}*"),
                Self::Type(body) => format!("&{body}"),
                Self::Float64 => format!("double"),
                Self::Float32 => format!("float"),
                Self::RawPointer => format!("ptr"),
                Self::Tuple(ts) => format!(
                    "({})",
                    ts.into_iter()
                        .map(|dt| format!("{dt}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}
