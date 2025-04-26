use std::fmt::{Debug, Display};

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
        return_type: Type,
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
    Call(Box<Node>, Vec<Node>),
    Return(Option<Box<Node>>),
    Break(Option<Box<Node>>),
    Continue(Option<Box<Node>>),
    Loop(Box<Node>),
    Use(UsePath),
    Import(Identifier),
    Identifier(String),
    Path(Vec<Identifier>),
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

#[derive(Default, PartialEq)]
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
impl Debug for RawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
            Tuple(list) => {
                return write!(
                    f,
                    "({})",
                    list.iter()
                        .map(|dt| format!("{}", dt.raw))
                        .collect::<Vec<std::string::String>>()
                        .join(", ")
                )
            }
            Other(path) => {
                return write!(
                    f,
                    "({})",
                    path.iter()
                        .map(|dt| format!("{}", dt.raw))
                        .collect::<Vec<std::string::String>>()
                        .join("::")
                )
            }
            Array(data_type, amount) => return write!(f, "[{};{}]", data_type.raw, amount.raw),
            Slice(data_type) => return write!(f, "[{}]", data_type.raw),

            SelfType => "self",
            Void => "void",
            Never => "never",
            Boolean => "bool",
            Char => "char",
            String => "str",

            Float32 => "f32",
            Float64 => "f64",

            USize => "usize",
            ISize => "isize",

            UInt(int) => return write!(f, "u{int}"),
            Int(int) => return write!(f, "i{int}"),
        };

        write!(f, "{str}")
    }
}
