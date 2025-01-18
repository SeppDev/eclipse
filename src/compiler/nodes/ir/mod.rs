use std::collections::HashMap;

use crate::compiler::{self, codegen};

mod display;

#[derive(Default)]
pub struct IRModule {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub key: String,
    pub parameters: Vec<(Type, String)>,
    pub return_type: Type,
    pub body: codegen::Source,
    pub variables: codegen::variables::VariablesMap,
    pub old_variables: HashMap<String, compiler::analyzer::Variable>,
}

pub enum BinaryOperationPrefix {
    Float,
    Signed,
    Unsigned,
}

pub enum BinaryOperation {
    Add(BinaryOperationPrefix),
    Subtract(BinaryOperationPrefix),
    Divide(BinaryOperationPrefix),
    Multiply(BinaryOperationPrefix),
    Remainder(BinaryOperationPrefix),
    LeftBitshift,
    RightBitshift(BinaryOperationPrefix),
}

pub enum Value {
    Reference(String),
    Register(String),
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
