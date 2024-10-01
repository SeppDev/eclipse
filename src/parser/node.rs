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

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Custom(String),
    Base(BaseType),
    Tuple(Vec<Type>),
}

#[allow(unused)]
#[derive(Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(isize),
    UInteger(usize),
    Boolean(bool),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<Expression>),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    // Tuple(Vec<Expression>),
}

#[allow(unused)]
#[derive(Debug)]
pub enum Node {
    Import(String),
    Call(Path, Vec<Expression>),
    Return(Option<Expression>),
    // Conditional((Expression, Expression), Vec<ASTNode>, Option<Vec<ASTNode>>),
    SetVariable(Path, Expression),
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
        generics: Vec<String>,
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

#[allow(unused)]
#[derive(Debug)]
pub struct ASTNode {
    pub indent: usize,
    pub lines: Range<usize>,
    pub node: Node,
}
impl ASTNode {
    pub fn new(indent: usize, lines: Range<usize>, node: Node) -> Self {
        Self {
            indent,
            lines,
            node,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    pub fn push(&mut self, path: &Self) {
        self.location.push(path.root.clone());
        for path in &path.location {
            self.location.push(path.clone());
        }
    }
    pub fn name(&self) -> &String {
        return match self.location.last() {
            Some(a) => a,
            None => &self.root
        }
    }
}
