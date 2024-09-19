use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseType {
    Int64,
    UInt64,

    Int32,
    UInt32,

    Int16,
    UInt16,

    Int8,
    UInt8,

    Boolean,

    Float64,
    Float32,
    // Intsize,
    // UIntsize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Custom(String),
    Base(BaseType),
    // StaticString,
    Tuple(Vec<Type>),
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(isize),
    UInteger(usize),
    // String(String),
    // Boolean(bool),
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
    GetVariable(Path),
    Call(Path, Vec<Expression>),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    // Tuple(Vec<Expression>),
}

#[derive(Debug)]
pub enum Node {
    Call(Path, Vec<Expression>),
    Return(Option<Expression>),
    // Conditional((Expression, Expression), Vec<ASTNode>, Option<Vec<ASTNode>>),
    SetVariable(Path, Expression),
    Struct {
        export: bool,
        name: String,
        generics: Vec<String>,
        body: Vec<(bool, String, Type)>,
    },
    Enum {
        export: bool,
        name: String,
        generics: Vec<String>,
        body: Vec<(String, Vec<Type>)>,
    },
    Import(String, bool),
    Loop {
        // condition
        body: Vec<ASTNode>,
    },
    Scope {
        is_unsafe: bool,
        body: Vec<ASTNode>,
    },
    Function {
        export: bool,
        is_unsafe: bool,
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Option<Type>,
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
    pub lines: Range<usize>,
    pub node: Node,
}
impl ASTNode {
    pub fn new(lines: Range<usize>, node: Node) -> Self {
        Self {
            lines,
            node
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub root: String,
    pub location: Vec<String>,
}
impl Path {
    pub fn new(root: String) -> Self {
        Self {
            root,
            location: Vec::new(),
        }
    }
    pub fn add(&mut self, name: String) {
        self.location.push(name)
    }
}