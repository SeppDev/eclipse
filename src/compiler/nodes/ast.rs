use std::fmt::Display;

use crate::{
    common::{path::Path, position::Located},
    compiler::lexer::token::TokenInfo,
};

pub type Node = Located<RawNode>;

pub type Parameter = Located<RawParameter>;
pub type Type = Located<RawType>;

pub type Identifier = Located<String>;
impl From<TokenInfo> for Identifier {
    fn from(value: TokenInfo) -> Self {
        Located { position: value.position, raw: value.string }
    }
}

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
        body: Box<Node>,
    },
    SetPath {
        path: Located<Path>,
        body: Box<Node>,
    },
    Declare {
        mutable: Option<TokenInfo>,
        name: Identifier,
        data_type: Option<Type>,
        node: Box<Node>,
    },
    Conditional {
        condition: Box<Node>,
        body: Box<Node>,
    },
    Return(Option<Box<Node>>),
    Break(Option<Box<Node>>),
    Continue(Option<Box<Node>>),
    Identifier(String),
    Integer(String),
    Float(String),
    Block(Vec<Node>),
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
impl Display for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => "void".to_string(),
                Self::Never => "never".to_string(),
                Self::UInt(int) => format!("u{int}"),
                Self::Int(int) => format!("i{int}"),
                _ => todo!(),
            }
        )
    }
}
