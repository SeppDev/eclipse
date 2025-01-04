use crate::compiler::{errors::CompileResult, path::Path};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum BaseType {
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

    Tuple(Vec<Type>),
    Array(usize, Box<Type>),
}
impl std::fmt::Display for BaseType {
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
                Self::Usize => format!("usize"),
                Self::ISize => format!("isize"),
                Self::Never => "!".to_string(),
                Self::Array(size, t) => format!("[{t}; {size}]"),
                Self::GetType(path) => format!("{path}"),
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

impl BaseType {
    pub fn is_basic(&self) -> bool {
        return self.is_number() || self.is_bool() || self.is_void();
    }
    pub fn is_number(&self) -> bool {
        return self.is_integer() || self.is_float();
    }
    pub fn is_void(&self) -> bool {
        matches!(&self, Self::Void)
    }
    pub fn is_integer(&self) -> bool {
        matches!(
            &self,
            Self::UInt(_) | Self::Int(_) | Self::ISize | Self::Usize
        )
    }
    pub fn is_float(&self) -> bool {
        matches!(&self, Self::Float32 | Self::Float64)
    }
    pub fn is_bool(&self) -> bool {
        matches!(&self, Self::Boolean)
    }
    pub fn is_array(&self) -> bool {
        matches!(&self, Self::Array(_, _))
    }
    // pub fn bytes(&self) -> usize {
    //     match &self {
    //         Self::ISize | Self::Usize => POINTER_WITH,
    //         Self::Int(bits) | BaseType::UInt(bits) => bits.div_ceil(8),
    //         Self::Float32 => 4,
    //         Self::Float64 => 8,
    //         Self::Never | BaseType::Void => 0,
    //         Self::Boolean => 1,
    //         Self::GetType(_) => todo!(),
    //         Self::Array(size, data_type) => data_type.bytes() * size,
    //         Self::Tuple(types) => {
    //             let mut size = 0;
    //             for data_type in types {
    //                 size += data_type.bytes()
    //             }
    //             size
    //         }
    //         Self::StaticString => todo!(),
    //     }
    // }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Type {
    pub base: BaseType,
    pub ref_state: ReferenceState,
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.ref_state, self.base)
    }
}
impl Type {
    pub fn new(base: BaseType) -> Self {
        let mut s = Self::default();
        s.base = base;
        return s;
    }
    pub fn boolean() -> Self {
        return Self::new(BaseType::Boolean);
    }
    // pub fn bytes(&self) -> usize {
    //     if self.pointers() > 0 {
    //         return POINTER_WITH;
    //     }
    //     self.base.bytes()
    // }
    pub fn pointers(&self) -> usize {
        return match self.ref_state {
            ReferenceState::Pointer(p) => p,
            ReferenceState::Shared | ReferenceState::Mutable => 1,
            ReferenceState::None => 0,
        };
    }
    pub fn reference(base: BaseType) -> Self {
        Self {
            ref_state: ReferenceState::Shared,
            base,
        }
    }
    pub fn pointer(base: BaseType) -> Self {
        Self {
            ref_state: ReferenceState::Pointer(1),
            base,
        }
    }
    pub fn void() -> Self {
        let mut s = Self::default();
        s.base = BaseType::Void;
        return s;
    }
    pub fn array_info(&self) -> (&Type, usize) {
        match &self.base {
            BaseType::Array(size, data_type) => return (data_type, *size),
            _ => return (self, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum ReferenceState {
    #[default]
    None,
    Shared,
    Mutable,
    Pointer(usize),
}
impl std::fmt::Display for ReferenceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "".to_string(),
                Self::Shared => "&".to_string(),
                Self::Mutable => "&mut ".to_string(),
                Self::Pointer(size) => "*".repeat(*size),
            }
        )
    }
}

impl Type {
    pub fn to_mutable(mut self) -> CompileResult<Type> {
        if matches!(self.ref_state, ReferenceState::None) {
            return Err(());
        }
        self.ref_state = ReferenceState::Mutable;
        return Ok(self);
    }
    pub fn to_reference(mut self) -> CompileResult<Type> {
        match self.ref_state {
            ReferenceState::None | ReferenceState::Shared => {
                self.ref_state = ReferenceState::Shared
            }
            _ => return Err(()),
        }
        return Ok(self);
    }
    pub fn add_pointer(mut self) -> CompileResult<Type> {
        match self.ref_state {
            ReferenceState::None => self.ref_state = ReferenceState::Pointer(1),
            ReferenceState::Pointer(p) => self.ref_state = ReferenceState::Pointer(p + 1),
            _ => return Err(()),
        }
        return Ok(self);
    }
    pub fn remove_pointer(mut self) -> CompileResult<Type> {
        match self.ref_state {
            ReferenceState::Pointer(p) if p > 1 => self.ref_state = ReferenceState::Pointer(p - 1),
            ReferenceState::Pointer(_) => self.ref_state = ReferenceState::None,
            _ => return Err(()),
        }
        return Ok(self);
    }
    pub fn dereference(mut self) -> CompileResult<Type> {
        match self.ref_state {
            ReferenceState::Mutable | ReferenceState::Shared => {
                self.ref_state = ReferenceState::None
            }
            _ => return Err(()),
        }
        return Ok(self);
    }
}
