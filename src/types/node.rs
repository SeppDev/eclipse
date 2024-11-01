#[derive(Debug)]
pub enum Node {
    // Use(bool, Path),
    // Conditional((Expression, Expression), Vec<ASTNode>, Option<Vec<ASTNode>>),
    Break,
    Expression(Expression),
    SetVariable(String, Expression),
    Return(Option<Expression>),
    Loop(Vec<ASTNode>),
    Struct {
        export: bool,
        name: String,
        generics: Option<Vec<String>>,
        body: Vec<(bool, String, Type)>,
    },
    Enum {
        export: bool,
        name: String,
        generics: Option<Vec<String>>,
        body: Vec<(String, Option<Type>)>,
    },
    Scope {
        is_unsafe: bool,
        body: Vec<ASTNode>,
    },
    Function {
        export: bool,
        is_unsafe: bool,
        name: String,
        generics: Vec<String>,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<ASTNode>,
    },
    DefineVariable {
        mutable: bool,
        name: String,
        data_type: Option<Type>,
        expression: Option<Expression>,
    },
}

#[derive(Debug)]
pub struct ASTNode {
    // pub indent: usize,
    pub lines: Range<usize>,
    pub node: Node,
}
impl ASTNode {
    pub fn new(lines: Range<usize>, node: Node) -> Self {
        Self {
            lines,
            node,
        }
    }
}

use std::ops::Range;

use super::{Path, Type};
#[derive(Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(bool, usize),
    Boolean(bool),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(String),
    Call(Path, Vec<Expression>),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}
