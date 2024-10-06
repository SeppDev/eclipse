use std::{ops::Range, path::PathBuf};

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
impl BaseType {
    fn is_integer(&self) -> bool {
        match &self {
            BaseType::Int8 => true,
            BaseType::UInt8 => true,
            BaseType::Int16 => true,
            BaseType::UInt16 => true,
            BaseType::Int32 => true,
            BaseType::UInt32 => true,
            BaseType::Int64 => true,
            BaseType::UInt64 => true,
            _ => false,
        }
    }
}

#[allow(unused)]
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

#[allow(unused)]
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
    Call(Path, Vec<Expression>),
    Return(Option<Expression>),
    Use(bool, Path),
    Import(bool, String),
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

#[allow(unused)]
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
            None => &self.root,
        };
    }
    pub fn to_pathbuf(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        buf.push(&self.root);
        for p in &self.location {
            buf.push(p);
        }
        return buf;
    }
    pub fn normalize(path: &PathBuf) -> Self {
        let mut components = path.components();
        let mut p = Path::new(String::from(
            components.next().unwrap().as_os_str().to_str().unwrap(),
        ));

        loop {
            let cmp = match components.next() {
                Some(a) => a,
                None => break,
            };
            p.add(String::from(cmp.as_os_str().to_str().unwrap()));
        }

        return p;
    }
}
