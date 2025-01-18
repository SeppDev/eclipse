use crate::compiler::{analyzer::Variables, path::Path};

mod types;
pub use types::Type;

use super::ast::{ArithmeticOperator, CompareOperator};

#[derive(Debug, Default)]
pub struct Typed<T: Default> {
    pub data_type: Type,
    pub raw: T,
}

pub type Expression = Typed<RawExpression>;
impl Expression {
    pub fn new(raw: RawExpression, data_type: Type) -> Self {
        Self { data_type, raw }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub mutable: bool,
    pub name: String,
    pub data_type: Type,
}

pub struct Field {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug)]
pub struct Function {
    pub key: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Node>,
    pub variables: Variables,
}

#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Unknown,
    Continue,
    Break,
    SetVariable {
        name: String,
        expression: Expression,
    },
    DeclareVariable {
        name: String,
        data_type: Type,
        expression: Expression,
    },
    IfStatement {
        expression: Expression,
        body: Vec<Node>,
        elseif: Vec<(Expression, Vec<Node>)>,
        else_body: Option<Vec<Node>>,
    },
    Loop {
        condition: Option<Expression>,
        body: Vec<Node>,
    },
    Scope(Vec<Node>),
    Call(String, Vec<Expression>),
    Return(Type, Option<Expression>),
    Result(Option<Expression>),
}

#[derive(Debug, Default)]
pub enum RawExpression {
    #[default]
    Unknown,
    Integer(String),
    Float(String),
    Boolean(bool),
    GetPath(Path),
    GetVariable(String),
    Field(Box<Expression>, String),
    Index(Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
    BinaryOperation(Box<Expression>, ArithmeticOperator, Box<Expression>),
    CompareOperation(Box<Expression>, CompareOperator, Box<Expression>),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Group(Box<Expression>),
    Minus(Box<Expression>),
    Not(Box<Expression>),
    Reference(Box<Expression>),
    DeReference(Box<Expression>),
    Struct(Path, Vec<(String, Expression)>),
}
