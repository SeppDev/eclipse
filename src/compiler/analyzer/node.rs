use crate::compiler::{parser::ExpressionInfo, types::Type};

use super::convert_type;

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
    Allocate(String, IRType),
    DeclareVariable(String, IRExpressionInfo),
    SetVariable(String, IRExpressionInfo),
    Call(String, Vec<ExpressionInfo>),
    Return(IRExpressionInfo),
}

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

#[derive(Debug)]
pub struct IRExpressionInfo {
    pub data_type: IRType,
    pub expression: IRExpression,
}
impl IRExpressionInfo {
    pub fn from(expression: IRExpression, data_type: &Type) -> Self {
        Self {
            expression,
            data_type: convert_type(data_type),
        }
    }
    pub fn void() -> Self {
        Self::from(IRExpression::Void, &Type::void())
    }
}

#[derive(Debug)]
pub enum IRType {
    Tuple(Vec<IRType>),
    Pointer(Box<IRType>),
    Integer(usize),
    Array(usize, Box<IRType>),
    Float,
    Double,
    Void,
}
impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IRType::Void => "void".to_string(),
                IRType::Double => "double".to_string(),
                IRType::Float => "float".to_string(),
                IRType::Array(size, t) => format!("[ {} x {} ]", size, t),
                IRType::Integer(bits) => format!("i{}", bits),
                IRType::Pointer(t) => format!("*{}", t),
                IRType::Tuple(types) => format!("{{ {} }}", {
                    let mut strings = Vec::new();
                    for t in types {
                        strings.push(format!("{}", t))
                    }
                    strings.join(", ")
                }),
            }
        )
    }
}
