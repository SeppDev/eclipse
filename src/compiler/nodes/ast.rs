use crate::{
    common::{path::Path, position::Located},
    compiler::lexer::token::TokenInfo,
};

pub type Parameter = Located<RawParameter>;
pub type Node = Located<RawNode>;
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
pub enum RawNode {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Node>,
    },
    SetPath {
        path: Located<Path>,
        body: Box<Node>,
    },
    Declare {
        name: Identifier,
        data_type: Option<Type>,
        expression: Option<Box<Node>>,
    },
    Identifier(String),
    Integer(String),
    Block(Vec<Node>),
    Return(Option<Box<Node>>),
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
