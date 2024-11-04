use crate::compiler::{lexer::Location, types::Type};

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<NodeInfo>,
    },
    SetVariable {
        name: String,
        expression: ExpressionInfo,
    },
    Variable {
        name: String,
        mutable: bool,
        data_type: Option<Type>,
        expression: ExpressionInfo
    },
    Expression(ExpressionInfo),
    Return(Option<ExpressionInfo>)
}

#[derive(Debug)]
pub struct NodeInfo {
    pub node: Node,
    pub location: Location
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(String),
    Call(String, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, Operator, Box<ExpressionInfo>)
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub expression: Expression,
    pub location: Location
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiply
}

#[derive(Debug)]
pub enum Value {
    Integer { minus: bool, integer: String },
}
