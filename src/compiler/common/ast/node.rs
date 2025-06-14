use std::fmt::Display;

use crate::compiler::common::operators::{ArithmethicOperator, CompareOperator, EqualsOperation};

use super::{Identifier, Location, Modifier, Node, Parameter, Type, UsePath};

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
        else_condition: Option<(Box<Node>, Box<Node>)>,
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
