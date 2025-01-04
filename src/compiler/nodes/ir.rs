mod value;
pub use value::*;

pub enum Instruction {
    Store {
        data_type: Type,
        value: Value<Primitive>,
        pointer: Value<Pointer>
    },
    Define {
        destination: String,
        operation: Operation
    }
}

pub enum Operation {
    Allocate(Type),
    Load()
}

pub enum Type {
    Int(usize),
    Bytes(usize),
    Array(usize, Box<Type>),
    Pointer(Box<Type>),
    Tuple(Vec<Type>),
    Reference(String),
    RawPointer,
    Boolean,
}
