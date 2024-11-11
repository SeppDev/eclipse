use crate::compiler::{parser::{Operator, Value}, path::Path, types::Type};

#[derive(Debug)]
pub struct IRFunction {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<IRNode>
}

#[derive(Debug)]
pub enum IRNode {
    DefineVariable(String, Type, IRExpression),
    SetVariable(String, IRExpression),
    Return(Option<IRExpression>)
}

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<IRExpression>),
    BinaryOperation(Box<IRExpression>, Operator, Box<IRExpression>),
    Tuple(Vec<IRExpression>),
    Minus(Box<IRExpression>),
    Pointer(Box<IRExpression>),
    Reference(Box<IRExpression>),
    Closure(Vec<(String, Type)>, Vec<IRNode>)
}

