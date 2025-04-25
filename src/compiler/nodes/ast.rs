use std::fmt::Display;

use crate::{common::position::LocatedAt, compiler::lexer::token::TokenInfo};

use super::shared::{ArithmethicOperator, CompareOperator, EqualsOperation};

pub type Node = LocatedAt<RawNode>;

pub type Location = LocatedAt<()>;
pub type Parameter = LocatedAt<RawParameter>;
pub type Type = LocatedAt<RawType>;

pub type Identifier = LocatedAt<String>;
impl From<TokenInfo> for Identifier {
    fn from(value: TokenInfo) -> Self {
        LocatedAt {
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

#[derive(Debug, PartialEq)]
pub enum UsePath {
    Ident(Identifier),
    Extend(Identifier, Box<UsePath>),
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
    // Enum {
    //     name: Identifier,
    //     items: Vec<Identifier>,
    // },
    // Struct {
    //     name: Identifier,
    //     fields: Vec<(Identifier, Type)>,
    // },
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

    Reference(Option<Identifier>, Box<Type>),

    Tuple(Vec<Type>),
    Array(Box<Type>, Identifier),
    Slice(Box<Type>),

    Other(Vec<Identifier>),
}
impl Into<Box<Type>> for RawType {
    fn into(self) -> Box<Type> {
        Box::new(self.into())
    }
}
impl Display for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawType::*;

        let str: &str = match self {
            Reference(lifetime, data_type) => match lifetime {
                Some(l) => return write!(f, "&'{} {}", l.raw, data_type.raw),
                None => return write!(f, "&{}", data_type.raw),
            },

            Void => "void",
            Never => "never",

            Float32 => "f32",
            Float64 => "f64",

            USize => "usize",
            ISize => "isize",

            UInt(int) if int.eq(&8) => "u8",
            UInt(int) if int.eq(&16) => "u16",
            UInt(int) if int.eq(&32) => "u32",
            UInt(int) if int.eq(&64) => "u64",

            Int(int) if int.eq(&8) => "i8",
            Int(int) if int.eq(&16) => "i16",
            Int(int) if int.eq(&32) => "i32",
            Int(int) if int.eq(&64) => "i64",
            _ => todo!(),
        };

        write!(f, "{str}")
    }
}
