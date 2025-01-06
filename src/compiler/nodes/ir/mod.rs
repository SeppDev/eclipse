mod value;
pub use value::*;

mod display;

#[derive(Default)]
pub struct IRModule {
    pub functions: Vec<Function>,
}


pub struct Function {
    pub key: String,
    pub parameters: Vec<(Type, String)>,
    pub return_type: Type,
    pub body: Vec<Instruction>
}

pub enum Instruction {
    Label(String),
    Return(Type, Option<Value>),
    Store {
        data_type: Type,
        value: Value,
        pointer: Value
    },
    Define {
        destination: String,
        operation: Operation
    }
}

pub enum Operation {
    Allocate(Type),
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
    Void,
}
