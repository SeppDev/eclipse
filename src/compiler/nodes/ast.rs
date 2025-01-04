use crate::compiler::{errors::Location, path::Path};

mod types;
pub use types::RawType;

#[derive(Debug, Default, Clone)]
pub struct Located<T> {
    pub location: Location,
    pub raw: T,
}

pub type Expression = Located<RawExpression>;
pub type Node = Located<RawNode>;
pub type Parameter = Located<RawParameter>;
pub type Field = Located<RawField>;
pub type Type = Located<RawType>;
pub type Identifier = Located<String>;
pub type LocatedPath = Located<Path>;

#[derive(Debug)]
pub struct RawParameter {
    pub mutable: bool,
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug)]
pub struct RawField {
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug, Default)]
pub enum RawNode {
    #[default]
    Unknown,
    Continue,
    Break,
    Enum {
        name: Identifier,
        fields: Vec<Identifier>,
    },
    Struct {
        name: Identifier,
        fields: Vec<Field>,
    },
    Function {
        name: Identifier,
        key: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Node>,
    },
    SetVariable {
        path: LocatedPath,
        expression: Option<Expression>,
    },
    DeclareVariable {
        name: Identifier,
        mutable: bool,
        data_type: Option<Type>,
        expression: Option<Expression>,
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
    Call(LocatedPath, Vec<Expression>),
    Return(Option<Expression>),
    Result(Option<Expression>),
    NameSpace(LocatedPath),
}

#[derive(Debug)]
pub enum RawExpression {
    Integer(String),
    Boolean(bool),
    Float(String),
    GetPath(LocatedPath),
    Field(Box<Expression>, Identifier),
    Index(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    BinaryOperation(Box<Expression>, ArithmeticOperator, Box<Expression>),
    CompareOperation(Box<Expression>, CompareOperator, Box<Expression>),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Minus(Box<Expression>),
    Not(Box<Expression>),
    Reference(Box<Expression>),
    DeReference(Box<Expression>),
    Struct(LocatedPath, Vec<(Identifier, Expression)>),
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Modulus,
    Plus,
    Subtract,
    Division,
    Multiply,
}

#[derive(Debug)]
pub enum CompareOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
}
