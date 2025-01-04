use super::{LocatedPath, Type};

#[derive(Debug, Default)]
pub enum RawType {
    #[default]
    Void,
    Never,

    Usize,
    ISize,
    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    Boolean,

    Reference(Box<Type>),

    Tuple(Vec<Type>),
    Array(usize, Box<Type>),
    GetType(LocatedPath),
}
impl std::fmt::Display for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean => "bool".to_string(),
                Self::Void => "void".to_string(),
                Self::Float32 => "f32".to_string(),
                Self::Float64 => "f64".to_string(),
                Self::UInt(bits) => format!("u{bits}"),
                Self::Int(bits) => format!("i{bits}"),
                Self::Reference(base) => format!("&{}", base.raw),
                Self::Usize => format!("usize"),
                Self::ISize => format!("isize"),
                Self::Never => "!".to_string(),
                Self::Array(size, t) => format!("[{}; {size}]", t.raw),
                Self::GetType(path) => format!("{}", path.raw),
                Self::Tuple(ts) => format!(
                    "({})",
                    ts.into_iter()
                        .map(|f| format!("{}", f.raw))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}
