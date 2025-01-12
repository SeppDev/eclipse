use std::collections::VecDeque;

use crate::compiler::codegen;

mod display;

#[derive(Default)]
pub struct IRModule {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub key: String,
    pub parameters: Vec<(Type, String)>,
    pub return_type: Type,
    pub body: VecDeque<Instruction>,
    pub variables: codegen::variables::VariablesMap
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
    Call(Type, String, Vec<(Type, Value)>),
    Constant(Type, Value)
}

pub enum Value {
    Reference(String),
    Integer(String),
    Float(String),
    Boolean(bool),
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
