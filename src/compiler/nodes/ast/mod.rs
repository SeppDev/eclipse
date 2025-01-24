use crate::{common::location::PositionRange, compiler::path::Path};

mod types;
pub use types::RawType;

#[derive(Debug, Default, Clone)]
pub struct Located<T> {
    pub position: PositionRange,
    pub raw: T,
}
impl<T> Located<T> {
    pub fn new(position: PositionRange, raw: T) -> Self {
        Self { position, raw }
    }
}

pub type Expression = Located<RawExpression>;
pub type Node = Located<RawNode>;
pub type Parameter = Located<RawParameter>;
pub type Field = Located<RawField>;
pub type Type = Located<RawType>;
pub type Layout = Located<RawLayout>;
pub type Function = Located<RawFunction>;

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

#[derive(Debug)]
pub enum Fields {
    List(Vec<Type>),
    Struct(Vec<Field>),
}

#[derive(Debug)]
pub enum RawLayout {
    Enum {
        name: Identifier,
        fields: Vec<(Identifier, Option<Fields>)>,
    },
    Struct {
        name: Identifier,
        fields: Fields,
    },
}

#[derive(Debug)]
pub struct RawFunction {
    pub key: String,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub enum RawNode {
    Continue,
    Break,
    SetPath(LocatedPath, Expression),
    DeclareVariable {
        name: Identifier,
        mutable: Option<Located<bool>>,
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
}

#[derive(Debug)]
pub enum RawExpression {
    Integer(String),
    Boolean(bool),
    Float(String),
    GetPath(LocatedPath),
    Field(Box<Expression>, Identifier),
    Index(Box<Expression>, Box<Expression>),
    CompareOperation(Box<Expression>, CompareOperator, Box<Expression>),
    ArithmeticOperation(Box<Expression>, ArithmeticOperator, Box<Expression>),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Group(Box<Expression>),
    Minus(Box<Expression>),
    Not(Box<Expression>),
    Increment(Box<Expression>),
    Decrement(Box<Expression>),
    Reference(Box<Expression>),
    DeReference(Box<Expression>),
    Invoke(Box<Expression>, Vec<Expression>),
    InvokeStruct(LocatedPath, Vec<(Identifier, Expression)>),
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LeftBitshift,
    RightBitshift,
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
