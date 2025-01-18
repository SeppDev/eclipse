use super::{BinaryOperation, BinaryOperationPrefix, Type, Value};
use std::fmt::Display;

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add(prefix) => format!("{}add", prefix.to_string(&self)),
                Self::Subtract(prefix) => format!("{}sub", prefix.to_string(&self)),

                Self::Multiply(prefix) => format!("{}mul", prefix.to_string(&self)),

                Self::Divide(prefix) => format!("{}div", prefix.to_string(&self)),
                Self::Remainder(prefix) => format!("{}rem", prefix.to_string(&self)),

                Self::LeftBitshift => format!("shl"),
                Self::RightBitshift(prefix) => format!(
                    "{}shr",
                    match prefix {
                        BinaryOperationPrefix::Signed => "a",
                        BinaryOperationPrefix::Unsigned => "l",
                        _ => panic!(""),
                    }
                ),
            }
        )
    }
}
impl BinaryOperationPrefix {
    fn to_string(&self, operation: &BinaryOperation) -> &str {
        match self {
            Self::Float => "f",
            Self::Unsigned
                if matches!(
                    operation,
                    BinaryOperation::Divide(..) | BinaryOperation::Remainder(..)
                ) =>
            {
                "u"
            }
            Self::Signed
                if matches!(
                    operation,
                    BinaryOperation::Divide(..) | BinaryOperation::Remainder(..)
                ) =>
            {
                "s"
            }
            Self::Signed => "",
            _ => "",
        }
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
