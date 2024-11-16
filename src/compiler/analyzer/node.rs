use crate::compiler::{parser::Operator, types::Type};

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<IRNode>,
}

#[derive(Debug)]
pub enum IRNode {
    Label(String),
    DeclareVariable(String, IRExpressionInfo),
    SetVariable(String, IRExpressionInfo),
    Return(IRExpressionInfo),
}

#[derive(Debug)]
pub enum IRExpression {
    Void,
    Allocate,
    Integer(String),
    Float(String),
    Boolean(bool),
    // StaticString
    GetVariable(String),
    Call(String, Vec<IRExpressionInfo>),
    BinaryOperation(Box<IRExpressionInfo>, Operator, Box<IRExpressionInfo>),
    Tuple(Vec<IRExpressionInfo>),
    Minus(Box<IRExpressionInfo>),
    Pointer(Box<IRExpressionInfo>),
    Closure(Vec<(String, Type)>, Vec<IRNode>),
}

#[derive(Debug)]
pub struct IRExpressionInfo {
    pub data_type: Type,
    pub expression: IRExpression,
}
impl IRExpressionInfo {
    pub fn from(expression: IRExpression, data_type: Type) -> Self {
        Self {
            expression,
            data_type,
        }
    }
    pub fn void() -> Self {
        Self::from(IRExpression::Void, Type::void())
    }
}
