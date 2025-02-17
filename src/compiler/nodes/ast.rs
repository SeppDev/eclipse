use crate::common::{path::Path, position::Located};

pub type Parameter = Located<RawParameter>;
pub type Expression = Located<RawExpression>;
pub type Type = Located<RawType>;

pub type Identifier = Located<String>;
pub type Keyword = Located<String>;
pub type Operator = Located<String>;

#[derive(Debug)]
pub struct RawParameter {
    pub reference: Operator,
    pub mutable: Keyword,
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug)]
pub enum RawExpression {
    Function(Identifier, Vec<Parameter>, Type, Box<Expression>),
    SetPath(Located<Path>, Box<Expression>),
    Declare(Identifier, Option<Keyword>, Option<Type>, Box<Expression>),
    Scope(Vec<Expression>),
    Return(Box<Expression>),
}

#[derive(Debug, Default)]
pub enum RawType {
    #[default]
    Void,
    Never,

    USize,
    ISize,
    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    Boolean,

    Reference(Box<Type>),

    Tuple(Vec<Type>),
    Array(Vec<Type>),
}
