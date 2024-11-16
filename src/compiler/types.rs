#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseType {
    Void,
    Never,

    Int64,
    UInt64,
    Int32,
    UInt32,
    Int16,
    UInt16,
    Int8,
    UInt8,

    Float32,
    Float64,

    Boolean,
    StaticString,
}
impl BaseType {
    pub fn size(&self) -> usize {
        use BaseType::*;

        match self {
            Void | StaticString | Never => 0,
            Int64 | UInt64 | Float64 => 8,
            Int32 | UInt32 | Float32 => 4,
            Int16 | UInt16 => 2,
            Int8 | UInt8 | Boolean => 1,
        }
    }
    pub fn is_bool(&self) -> bool {
        self == &Self::Boolean
    }
    pub fn is_integer(&self) -> bool {
        self == &Self::Boolean
    }
}
impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean => "bool",
                Self::Void => "void",
                Self::Float32 => "f32",
                Self::Float64 => "f64",
                Self::Int8 => "i8",
                Self::UInt8 => "u8",
                Self::Int16 => "i16",
                Self::UInt16 => "u16",
                Self::Int32 => "i32",
                Self::UInt32 => "u32",
                Self::Int64 => "i64",
                Self::UInt64 => "u64",
                Self::Never => "!",
                Self::StaticString => "static_string",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    // Custom(String),
    Unkown,
    Base(BaseType),
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Pointer(Box<Type>),
    Reference(Box<Type>),
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unkown => write!(f, "{{unkown}}"),
            Self::Base(base) => write!(f, "{}", base),
            Self::Array(t, size) => write!(f, "[{}; {}]", t, size),
            Self::Pointer(t) => write!(f, "*{}", t),
            Self::Reference(t) => write!(f, "&{}", t),
            Self::Tuple(types) => write!(
                f,
                "({})",
                types
                    .into_iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
impl Type {
    // The bool in the return type is true if the integer is signed
    pub fn integer_info(&self) -> Option<(bool, usize)> {
        use BaseType::*;

        return Some(match &self {
            Type::Base(base) => match base {
                UInt8 => (false, 8),
                Int8 => (true, 8),

                UInt16 => (false, 16),
                Int16 => (true, 16),

                UInt32 => (false, 32),
                Int32 => (true, 32),

                UInt64 => (false, 64),
                Int64 => (true, 64),
                _ => return None,
            },
            _ => return None,
        });
    }
    pub fn void() -> Self {
        self::Type::Base(BaseType::Void)
    }
    // pub fn size(&self) -> usize {
    //     match self {
    //         Self::Base(base) => base.size(),
    //         Self::Array(t, size) => t.size() * size,
    //         Self::Tuple(types) => {
    //             let mut size = 0;
    //             for x in types {
    //                 size += x.size();
    //             }
    //             size
    //         }
    //         _ => todo!(),
    //     }
    // }
    pub fn is_integer(&self) -> bool {
        use BaseType::*;
        match &self {
            Type::Base(base) => match base {
                Int8 | UInt8 | Int16 | UInt16 | Int32 | UInt32 | Int64 | UInt64 => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        use BaseType::*;
        match &self {
            Type::Base(base) => match base {
                Float32 | Float64 => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_void(&self) -> bool {
        return match &self {
            Self::Base(base) => match base {
                BaseType::Void => true,
                _ => false,
            },
            _ => false,
        };
    }
}
