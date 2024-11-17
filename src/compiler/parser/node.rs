use crate::compiler::{
    errors::Location,
    path::Path,
    types::{BaseType, Type},
};

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
        parameters: Vec<(String, Type)>,
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
        else_expression: Option<Vec<NodeInfo>>,
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

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, Operator, Box<ExpressionInfo>),
    Tuple(Vec<ExpressionInfo>),
    Minus(Box<ExpressionInfo>),
    Not(Box<ExpressionInfo>),
    Pointer(Box<ExpressionInfo>),
    Reference(Box<ExpressionInfo>), // Field(Box<ExpressionInfo>, Box<ExpressionInfo>)
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub location: Location,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiply,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
}

// #[derive(Debug)]
// pub enum ArithmeticOperator {
//     Plus,
//     Minus,
//     Division,
//     Multiply,
// }
// #[derive(Debug)]
// pub enum CompareOperator {
//     Equals,
//     NotEquals,
//     GreaterThan,
//     GreaterThanOrEquals,
//     LessThan,
//     LessThanOrEquals,
// }

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(String),
    Float(String),
    StaticString(String),
}
impl Value {
    pub fn default_type(&self) -> Type {
        return Type::Base(match self {
            Self::Boolean(_) => BaseType::Boolean,
            Self::Float(_) => BaseType::Float64,
            Self::Integer(_) => BaseType::Int32,
            Self::StaticString(_) => BaseType::StaticString,
        });
    }
}
