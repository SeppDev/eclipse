use std::fmt::{Debug, Display};

use super::{Identifier, Type};

#[derive(Default, PartialEq, Clone)]
pub enum RawType {
    #[default]
    Void,
    Never,

    String,
    Char,

    USize,
    ISize,
    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    SelfType,

    Boolean,

    Ref(Option<Identifier>, Box<Type>),
    RefMut(Option<Identifier>, Box<Type>),

    // Box(Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>, Identifier),
    Slice(Box<Type>),

    Other(Vec<Identifier>),
}
impl Into<Box<Type>> for RawType {
    fn into(self) -> Box<Type> {
        Box::new(self.into())
    }
}
impl Debug for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Display for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawType::*;

        let str: &str = match self {
            Ref(lifetime, data_type) => match lifetime {
                Some(l) => return write!(f, "&'{} {}", l.raw, data_type.raw),
                None => return write!(f, "&{}", data_type.raw),
            },
            RefMut(lifetime, data_type) => match lifetime {
                Some(l) => return write!(f, "&'{} mut {}", l.raw, data_type.raw),
                None => return write!(f, "&mut {}", data_type.raw),
            },
            Tuple(list) => {
                return write!(
                    f,
                    "({})",
                    list.iter()
                        .map(|dt| format!("{}", dt.raw))
                        .collect::<Vec<std::string::String>>()
                        .join(", ")
                )
            }
            Other(path) => {
                return write!(
                    f,
                    "({})",
                    path.iter()
                        .map(|dt| format!("{}", dt.raw))
                        .collect::<Vec<std::string::String>>()
                        .join("::")
                )
            }
            Array(data_type, amount) => return write!(f, "[{}; {}]", data_type.raw, amount.raw),
            Slice(data_type) => return write!(f, "[{}]", data_type.raw),
            // Box(data_type) => return write!(f, "Box<{}>", data_type.raw),
            SelfType => "self",
            Void => "void",
            Never => "never",
            Boolean => "bool",
            Char => "char",
            String => "str",

            Float32 => "f32",
            Float64 => "f64",

            USize => "usize",
            ISize => "isize",

            UInt(int) => return write!(f, "u{int}"),
            Int(int) => return write!(f, "i{int}"),
        };

        write!(f, "{str}")
    }
}
