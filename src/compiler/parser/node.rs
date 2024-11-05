use crate::compiler::{lexer::Location, path::Path, types::Type};

#[derive(Debug)]
pub enum Node {
    Scope(Vec<NodeInfo>),
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
    Call(Path, Vec<ExpressionInfo>),
    Return(Option<ExpressionInfo>)
}

#[derive(Debug)]
pub struct NodeInfo {
    pub location: Location,
    pub node: Node,
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, Operator, Box<ExpressionInfo>)
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub location: Location,
    pub expression: Expression,
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
