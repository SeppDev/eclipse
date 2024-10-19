use crate::parser::{Path, Type, Value};

#[derive(Debug)]
pub struct Program {}

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    DefineVariable(Type, String),
    GetVariable(Path),
}

#[derive(Debug)]
pub enum IRNode {
    Return(Option<IRExpression>),
    Scope {
        is_unsafe: bool,
        body: Vec<IRNode>,
    },
    DefineVariable {
        name: String,
        data_type: Type,
        expression: IRExpression,
    },
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<IRNode>,
}

pub struct IRModule {
    pub body: Vec<Function>
}
impl IRModule {
    pub fn new() -> Self {
        Self {
            body: Vec::new()
        }
    }
}

