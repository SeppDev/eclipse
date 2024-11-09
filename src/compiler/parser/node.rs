use crate::compiler::{lexer::Location, path::Path, types::Type};

#[derive(Debug)]
pub struct Function {
    pub public: bool,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<NodeInfo>,
}

#[derive(Debug)]
pub enum Node {
    Scope(Vec<NodeInfo>),
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
    NameSpace {
        public: bool,
        static_path: Path,
    },
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
    BinaryOperation(Box<ExpressionInfo>, Operator, Box<ExpressionInfo>),
    Tuple(Vec<ExpressionInfo>),
    Minus(Box<ExpressionInfo>),
    Pointer(Box<ExpressionInfo>),
    Reference(Box<ExpressionInfo>), // Field(Box<ExpressionInfo>, Box<ExpressionInfo>)
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
    Multiply,
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(String),
    Float(String),
    String(String),
}
