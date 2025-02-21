use crate::{
    common::{path::Path, position::Located},
    compiler::lexer::token::TokenInfo,
};

pub type Parameter = Located<RawParameter>;
pub type Expression = Located<RawExpression>;
pub type Type = Located<RawType>;

pub type Identifier = Located<String>;

#[derive(Debug)]
pub struct RawParameter {
    pub reference: Option<TokenInfo>,
    pub mutable: Option<TokenInfo>,
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug)]
pub enum RawExpression {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Box<Expression>,
    },
    SetPath {
        path: Located<Path>,
        body: Box<Expression>,
    },
    Declare {
        name: Identifier,
        data_type: Option<Type>,
        expression: Option<Box<Expression>>,
    },
    // DeclareConst {}
    Integer(String),
    Block(Vec<Expression>),
    Return(Box<Option<Expression>>),
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
