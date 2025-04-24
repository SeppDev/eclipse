use std::fmt::Display;

use crate::{common::position::Located, compiler::lexer::token::TokenInfo};

use super::shared::{ArithmethicOperator, CompareOperator, EqualsOperation};

pub type Node = Located<RawNode>;

pub type Location = Located<()>;
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

#[derive(Debug, PartialEq)]
pub struct RawParameter {
    pub reference: Option<TokenInfo>,
    pub mutable: Option<TokenInfo>,
    pub name: Identifier,
    pub data_type: Type,
}

pub type UsePath = Located<RawUsePath>;
#[derive(Debug, PartialEq)]
pub enum RawUsePath {
    String(String),
    List(Vec<UsePath>),
}

#[derive(Debug, PartialEq)]
pub enum RawNode {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Box<Node>,
    },
    SetPath {
        path: Identifier,
        operation: EqualsOperation,
        value: Box<Node>,
    },
    Declare {
        mutable: Option<Location>,
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
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    If {
        condition: Box<Node>,
        body: Box<Node>,
    },
    Field(Box<Node>, Identifier),
    Call(String, Vec<Node>),
    Return(Option<Box<Node>>),
    Break(Option<Box<Node>>),
    Continue(Option<Box<Node>>),
    Loop(Box<Node>),
    Use(UsePath),
    Import(Identifier),
    Identifier(String),
    String(String),
    Bool(bool),
    Integer(String),
    Minus(Box<Node>),
    Float(String),
    Tuple(Vec<Node>),
    Wrapped(Option<Box<Node>>),
    Block(Vec<Node>),
    Enum {
        name: Identifier,
        items: Vec<Identifier>,
    },
    Struct {
        name: Identifier,
        fields: Vec<(Identifier, Type)>,
    },
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawNode::*;

        let string = match &self.raw {
            ArithmethicOperation {
                left,
                right,
                operator,
            } => format!("{left} {operator} {right}"),
            CompareOperation {
                left,
                right,
                operator,
            } => format!("{left} {operator} {right}"),
            Field(node, field) => format!("{node}.{}", field.raw),
            Integer(s) | Identifier(s) | Float(s) => s.into(),
            s => format!("{s:?}"),
        };

        write!(f, "{string}")
    }
}
impl Into<Box<Node>> for RawNode {
    fn into(self) -> Box<Node> {
        Box::new(self.into())
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum RawType {
    #[default]
    Void,
    Never,

    String,
    Char,

    USize,
    ISize,
    UInt(usize),
    Int(usize),

    Float32,
    Float64,

    SelfType,

    Boolean,

    Reference(Box<Type>),

    Tuple(Vec<Type>),
    Array(Vec<Type>),
}
impl Into<Box<Type>> for RawType {
    fn into(self) -> Box<Type> {
        Box::new(self.into())
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
