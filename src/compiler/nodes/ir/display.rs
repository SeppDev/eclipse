use super::{Instruction, Operation, Type, Value};
use std::fmt::Display;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Label(l) => format!("{l}"),
                Self::Define {
                    destination,
                    operation,
                } => format!("\t%{destination} = {operation}"),
                Self::Store {
                    data_type,
                    value,
                    pointer,
                } => format!("\tstore {data_type} {value}, ptr {pointer}"),
                Self::Return(data_type, value) => match value {
                    Some(val) => format!("\tret {data_type} {val}"),
                    None => format!("\tret {data_type}")
                },
            }
        )
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Allocate(data_type) => format!("alloca {data_type}"),
                Self::Load(data_type, value) => format!("load {data_type}, ptr {value}"),
                
                Self::Constant(data_type, value) => format!("constant {data_type} {value}"),
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
                Self::Reference(body) => format!("%{body}"),
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
