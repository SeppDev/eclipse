#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Type {
    #[default]
    Void,
    Never,
    
    Isize,
    Usize,
    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    Boolean,

    Reference(Box<Type>),
    Tuple(Vec<Type>),
    Array(usize, Box<Type>),
}

impl std::fmt::Display for Type {
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
                Self::Reference(base) => format!("&{base}"),
                Self::Usize => format!("usize"),
                Self::Isize => format!("isize"),
                Self::Never => "!".to_string(),
                Self::Array(size, t) => format!("[{t}; {size}]"),
                Self::Tuple(ts) => format!(
                    "({})",
                    ts.into_iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}
