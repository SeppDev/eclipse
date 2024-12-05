use super::errors::CompileResult;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum BaseType {
    #[default]
    Never,
    Void,

    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    Boolean,
    StaticString(usize),

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
                Self::Never => "!".to_string(),
                Self::StaticString(_) => "str".to_string(),
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
    pub fn reference(base: BaseType) -> Self {
        let mut s = Self::default();
        s.base = base;
        let _ = s.add_reference();
        return s;
    }
    pub fn pointer(base: BaseType) -> Self {
        let mut s = Self::default();
        s.base = base;
        let _ = s.add_pointer();
        return s;
    }
    pub fn void() -> Self {
        let mut s = Self::default();
        s.base = BaseType::Void;
        return s;
    }
    pub fn array_info(self) -> (Type, usize) {
        match self.base {
            BaseType::Array(size, data_type) => return (*data_type, size),
            base => panic!("{base} is not an array"),
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
pub trait ReferenceManager {
    fn add_pointer(&mut self) -> CompileResult<()>;
    fn add_reference(&mut self) -> CompileResult<()>;
}

impl Type {
    pub fn to_reference(mut self) -> CompileResult<Type> {
        self.add_reference()?;
        return Ok(self);
    }
    pub fn to_pointer(mut self) -> CompileResult<Type> {
        self.add_pointer()?;
        return Ok(self);
    }
    pub fn is_reference(&self) -> bool {
        return matches!(
            self.ref_state,
            ReferenceState::Mutable | ReferenceState::Shared
        );
    }
    pub fn is_pointer(&self) -> Option<usize> {
        match &self.ref_state {
            ReferenceState::Pointer(p) => Some(p.clone()),
            _ => None,
        }
    }
}

impl ReferenceManager for Type {
    fn add_reference(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None | ReferenceState::Shared => {
                self.ref_state = ReferenceState::Shared
            }
            _ => return Err(()),
        }
        return Ok(());
    }
    fn add_pointer(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None => self.ref_state = ReferenceState::Pointer(1),
            ReferenceState::Pointer(p) => self.ref_state = ReferenceState::Pointer(p + 1),
            _ => return Err(()),
        }
        return Ok(());
    }
}
