use std::{fmt, ops::Range, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseType {
    Void,

    Int64,
    UInt64,

    Int32,
    UInt32,

    Int16,
    UInt16,

    Int8,
    UInt8,

    Boolean,

    Float16,
    Float32,
    Float64,
    Float128
    // Intsize,
    // UIntsize,
}
impl BaseType {
    fn is_integer(&self) -> bool {
        use BaseType::*;
        match &self {
            Int8 | UInt8 | Int16 | UInt16 | Int32 | UInt32 | Int64 | UInt64 => true,
            _ => false,
        }
    }
    fn is_signed(&self) -> bool {
        use BaseType::*;
        match &self {
            Int8 | Int16 | Int32 | Int64 => true,
            _ => false,
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Custom(String),
    Base(BaseType),
    Tuple(Vec<Type>),
}
impl Type {
    pub fn is_integer(&self) -> bool {
        match &self {
            Type::Base(base) => base.is_integer(),
            _ => false,
        }
    }
}


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
    // Tuple(Vec<Expression>),
}

#[derive(Debug)]
pub enum Node {
    // Use(bool, Path),
    // Conditional((Expression, Expression), Vec<ASTNode>, Option<Vec<ASTNode>>),
    // Loop(Vec<ASTNode>),
    Expression(Expression),
    SetVariable(String, Expression),
    Return(Option<Expression>),
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
    pub components: Vec<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.components.join("/"))
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn from(root: String) -> Self {
        let mut path = Self::new();
        path.add(root);
        return path;
    }
    pub fn join(&self, seperator: String) -> String {
        self.components.join(&seperator)
    }
    pub fn add(&mut self, name: String) {
        self.components.push(name)
    }
    pub fn push(&mut self, path: &Self) {
        for path in &path.components {
            self.components.push(path.clone());
        }
    }
    pub fn to_pathbuf(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for p in &self.components {
            buf.push(p);
        }
        return buf;
    }
    pub fn from_pathbuf(path: &PathBuf) -> Self {
        let components = path.components();
        let mut path = Path::new();

        for component in components.into_iter() {
            path.add(String::from(component.as_os_str().to_str().unwrap()));
        }

        return path;
    }
}
