use crate::{parser::{Type, Value}, Operator};

use super::ModuleTypes;

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(String),
    BinaryOperation(Box<IRExpression>, Operator, Box<IRExpression>),
    Call(String, Vec<(IRExpression, Type)>),
}

#[derive(Debug)]
pub enum IRNode {
    // Scope {
    //     is_unsafe: bool,
    //     body: Vec<IRNode>,
    // },
    Call(String, Type, Vec<IRExpression>),
    Return(Option<IRExpression>),
    SetVariable(String, Type, IRExpression),
    DefineVariable(String, Type, IRExpression),
    Loop(Vec<IRNode>),
    // Break,
}
impl IRNode {
    pub fn is_return(&self) -> bool {
        match self {
            Self::Return(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub nodes: Vec<IRNode>,
}

#[derive(Debug)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRProgram {
    pub types: ModuleTypes,
    pub modules: Vec<IRModule>,
}
