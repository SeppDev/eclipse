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
        expression: ExpressionInfo,
    },
    Call(Path, Vec<ExpressionInfo>),
    Return(Option<ExpressionInfo>),
    NameSpace(Path)
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct NodeInfo {
    pub location: Location,
    pub node: Node,
}
impl std::fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, Operator, Box<ExpressionInfo>),
    Tuple(Vec<ExpressionInfo>),
    Minus(Box<ExpressionInfo>),
    Pointer(Box<ExpressionInfo>),
    Reference(Box<ExpressionInfo>), // Field(Box<ExpressionInfo>, Box<ExpressionInfo>)
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub location: Location,
    pub expression: Expression,
}
impl std::fmt::Display for ExpressionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiply,
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(String),
    Float(String),
    String(String),
}
