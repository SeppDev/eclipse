use std::collections::HashMap;

use crate::parser::{Type, Value};

#[derive(Debug)]
pub struct Program {}

#[derive(Debug)]
pub enum IRExpression {
    Value(Type, Value),
    DefineVariable(Type, String),
    GetVariable(Type, String),
}
impl IRExpression {
    pub fn parse_type(&self) -> Type {
        return match self {
            IRExpression::Value(t, _) => t,
            IRExpression::GetVariable(t, _) => t,
            IRExpression::DefineVariable(t, _) => t,
        }
        .clone();
    }
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
        // data_type: Type,
        expression: IRExpression,
    },
}

#[derive(Debug)]
pub struct IRFunction {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub nodes: Vec<IRNode>,
}

// #[derive(Debug, Default)]
// pub struct IRModule {
//     pub submodules: HashMap<String, IRModule>,
//     pub body: HashMap<String, IRFunction>,
// }
// impl IRModule {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
