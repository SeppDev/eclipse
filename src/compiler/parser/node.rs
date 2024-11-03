use crate::compiler::types::Type;

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Node>,
    },
    SetVariable {
        name: String,
        expression: Expression,
    },
    Variable {
        name: String,
        mutable: bool,
        data_type: Option<Type>,
        expression: Expression
    },
    Expression(Expression),
    Return(Option<Expression>)
}

pub struct NodeInfo {
    pub node: Node,
    
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(String),
    Call(String, Vec<Expression>),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>)
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
