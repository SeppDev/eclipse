use crate::compiler::{parser::Operator, types::Type};

#[allow(unused)]
#[derive(Debug)]
pub struct IRFunction {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<IRNode>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum IRNode {
    Label(String),
    DeclareVariable(String, IRExpressionInfo),
    SetVariable(String, IRExpressionInfo),
    Return(IRExpressionInfo),
}


#[allow(unused)]
#[derive(Debug)]
pub enum IRExpression {
    Void,
    Allocate(Type),
    Integer(String),
    Float(String),
    Boolean(bool),
    GetVariable(String),
    Call(String, Vec<IRExpressionInfo>),
    BinaryOperation(Box<IRExpressionInfo>, Operator, Box<IRExpressionInfo>),
    Tuple(Vec<IRExpressionInfo>),
    Minus(Box<IRExpressionInfo>),
    Pointer(Box<IRExpressionInfo>),
    Closure(Vec<(String, Type)>, Vec<IRNode>),
}

#[allow(unused)]
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
