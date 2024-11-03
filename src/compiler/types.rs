#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseType {
    Void,

    Int64,
    UInt64,
    Int32,
    UInt32,
    Int16,
    UInt16,
    Int8,
    UInt8,

    Boolean,

    Float32,
    Float64,
    // Float128
    // Intsize,
    // UIntsize,
}
impl BaseType {
    pub fn size(&self) -> usize {
        use BaseType::*;
        
        match self {
            Void => 0,
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


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    // Custom(String),
    Base(BaseType),
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
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
            }
            _ => return None,
        });
    }
    pub fn size(&self) -> usize {
        match self {
            Self::Base(base) => base.size(),
            Self::Array(t, size) => t.size() * size,
            Self::Tuple(types) => {
                let mut size = 0;
                for x in types {
                    size += x.size();
                }
                size
            }
        }
    }
    pub fn is_integer(&self) -> bool {
        use BaseType::*;
        match &self {
            Type::Base(base) => match base {
                Int8 | UInt8 | Int16 | UInt16 | Int32 | UInt32 | Int64 | UInt64 => true,
                _ => false,
            }
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        use BaseType::*;
        match &self {
            Type::Base(base) => match base {
                Float32 | Float64 => true,
                _ => false,
            }
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        use BaseType::*;
        match &self {
            Type::Base(base) => match base {
                Boolean => true,
                _ => false,
            }
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
        }
    }
}