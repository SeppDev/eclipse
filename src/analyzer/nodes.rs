use crate::parser::{Type, Value};

use super::ModuleTypes;

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(String),
    Call(String, Vec<IRExpression>),
}

#[derive(Debug)]
pub enum IRNode {
    // Scope {
    //     is_unsafe: bool,
    //     body: Vec<IRNode>,
    // },
    Expression(IRExpression, Type),
    Return(Option<IRExpression>),
    DefineVariable {
        name: String,
        data_type: Type,
        expression: IRExpression,
    },
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
