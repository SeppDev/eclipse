use std::fmt::Display;

use crate::compiler::{
    errors::{CompileResult, Location},
    path::Path,
    types::{BaseType, ReferenceManager, ReferenceState, Type},
};

#[allow(unused)]
#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Uknown,
    Enum {
        name: String,
        fields: Vec<String>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Function {
        export: bool,
        name: String,
        key: String,
        parameters: Vec<(bool, String, Type)>,
        return_type: Type,
        body: Vec<NodeInfo>,
    },
    Scope(Vec<NodeInfo>),
    SetVariable {
        name: String,
        expression: Option<ExpressionInfo>,
    },
    DeclareVariable {
        name: String,
        mutable: bool,
        data_type: Option<Type>,
        expression: Option<ExpressionInfo>,
    },
    IfStatement {
        expression: (ExpressionInfo, Vec<NodeInfo>),
        elseif: Vec<(ExpressionInfo, Vec<NodeInfo>)>,
        else_body: Option<Vec<NodeInfo>>,
    },
    Call(Path, Vec<ExpressionInfo>),
    Return(Option<ExpressionInfo>),
    NameSpace {
        public: bool,
        static_path: Path,
    },
}

#[derive(Debug, Default)]
pub struct NodeInfo {
    pub location: Location,
    pub node: Node,
}
impl NodeInfo {
    pub fn void() -> Self {
        Self {
            location: Location::void(),
            node: Node::Uknown
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, ArithmeticOperator, Box<ExpressionInfo>),
    CompareOperation(Box<ExpressionInfo>, CompareOperator, Box<ExpressionInfo>),
    Array(Vec<ExpressionInfo>),
    Tuple(Vec<ExpressionInfo>),
    Minus(Box<ExpressionInfo>),
    Not(Box<ExpressionInfo>),
    // Field(Box<ExpressionInfo>, Box<ExpressionInfo>)
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub location: Location,
    pub ref_state: ReferenceState,
    pub expression: Expression,
}
impl ReferenceManager for ExpressionInfo {
    fn add_pointer(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None => self.ref_state = ReferenceState::Pointer(1),
            ReferenceState::Pointer(p) => self.ref_state = ReferenceState::Pointer(p + 1),
            _ => return Err(())
        }
        return Ok(())
    }
    fn add_reference(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None | ReferenceState::Shared => self.ref_state = ReferenceState::Shared,
            _ => return Err(())
        }
        return Ok(())
    }
} 

#[derive(Debug)]
pub enum ArithmeticOperator {
    // Modulus
    Plus,
    Minus,
    Division,
    Multiply,
}
impl Display for ArithmeticOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            ArithmeticOperator::Plus => "add",
            ArithmeticOperator::Minus => "sub",
            ArithmeticOperator::Multiply => "mul",
            ArithmeticOperator::Division => "div",
        })
    }
}

#[derive(Debug)]
pub enum CompareOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
}
impl Display for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "icmp {}", match &self {
            Self::Equals => "eq",
            Self::NotEquals => "ne",
            Self::GreaterThan => "sgt",
            Self::GreaterThanOrEquals => "sge",
            Self::LessThan => "slt",
            Self::LessThanOrEquals => "sle",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(String),
    Float(String),
    StaticString(String),
}
impl Value {
    pub fn default_type(&self) -> Type {
        let mut a = Type::default();
        match &self {
            Self::Boolean(_) => a.base = BaseType::Boolean,
            Self::Float(_) => a.base = BaseType::Float64,
            Self::Integer(_) => a.base = BaseType::Int(32),
            Self::StaticString(_) => todo!()
        }
        return a;
    }
}
