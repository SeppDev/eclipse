use crate::compiler::types::Type;

#[allow(unused)]
#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
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
    Allocate,

    Integer(String),
    Float(String),
    Boolean(bool),

    Minus(Box<IRExpressionInfo>),
    Add(Box<IRExpression>, Box<IRExpression>),

    GetVariable(String),
    Call(String, Vec<IRExpressionInfo>),
    Tuple(Vec<IRExpressionInfo>),
    Pointer(Box<IRExpressionInfo>),
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
