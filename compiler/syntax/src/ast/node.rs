use super::{Attribute, Identifier, Location, Modifier, Node, Parameter, Type, UsePath};
use crate::operators::{EqualsOperation, Operator};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum RawNode {
    Modifiers(Vec<Modifier>, Box<Node>),
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
        conditions: Vec<(Node, Node)>,
        else_condition: Option<Box<Node>>,
    },
    Operation {
        left: Box<Node>,
        right: Box<Node>,
        operator: Operator,
    },
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    Attribute(Attribute),
    Field(Box<Node>, Box<Node>),
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

#[derive(Debug, PartialEq)]
pub enum RawAttribute {
    Simple(Identifier),
}

impl Display for RawNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawNode::*;

        let string = match &self {
            Operation {
                left,
                right,
                operator,
            } => format!("{left} {operator} {right}"),
            Field(node, field) => format!("{node}.{field}"),
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
