use std::fmt::Display;

use crate::{
    common::position::{Located, PositionRange},
    compiler::lexer::token::TokenInfo,
};

use super::shared::{ArithmethicOperator, CompareOperator};

pub type Node = Located<RawNode>;

pub type Parameter = Located<RawParameter>;
pub type Type = Located<RawType>;

pub type Identifier = Located<String>;
impl From<TokenInfo> for Identifier {
    fn from(value: TokenInfo) -> Self {
        Located {
            position: value.position,
            raw: value.string,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawParameter {
    pub reference: Option<TokenInfo>,
    pub mutable: Option<TokenInfo>,
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RawNode {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Box<Node>,
    },
    SetPath {
        path: Identifier,
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
    ArithmethicOperation {
        left: Box<Node>,
        right: Box<Node>,
        operator: ArithmethicOperator,
    },
    CompareOperation {
        left: Box<Node>,
        right: Box<Node>,
        operator: CompareOperator,
    },
    Field(Box<Node>, String),
    Call(String, Vec<Node>),
    Return(Option<Box<Node>>),
    Break(Option<Box<Node>>),
    Continue(Option<Box<Node>>),
    Identifier(String),
    String(String),
    Bool(bool),
    Integer(String),
    MinusInteger(Box<Node>),
    Float(String),
    Block(Vec<Node>),
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawNode::*;

        let string = match &self.raw {
            ArithmethicOperation {
                left,
                right,
                operator,
            } => &format!("{left} {operator} {right}"),
            CompareOperation {
                left,
                right,
                operator,
            } => &format!("{left} {operator} {right}"),
            Integer(s) | Identifier(s) => s,
            s => &format!("{s:?}"),
        };

        write!(f, "{string}")
    }
}
impl Into<Node> for RawNode {
    fn into(self) -> Node {
        Node::new(self, PositionRange::default())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
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
impl Into<Type> for RawType {
    fn into(self) -> Type {
        Type::new(self, PositionRange::default())
    }
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
