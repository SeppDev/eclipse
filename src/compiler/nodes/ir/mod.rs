mod value;
use std::collections::VecDeque;

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
    pub body: VecDeque<Instruction>
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
    Load(Type, Value),
    Constant(Type, Value)
}


#[derive(Clone)]
pub enum Type {
    Int(usize),
    Bytes(usize),
    Array(usize, Box<Type>),
    Pointer(Box<Type>),
    Tuple(Vec<Type>),
    Type(String),
    RawPointer,
    Boolean,
    Float32,
    Float64,
    Void,
}
