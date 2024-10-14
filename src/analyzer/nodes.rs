use std::collections::HashMap;

use crate::parser::{Path, Type, Value};

#[allow(unused)]
#[derive(Debug)]
pub struct Program {}

#[allow(unused)]
#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(Path),
}

#[allow(unused)]
#[derive(Debug)]
pub enum IRNode {
    Return(Option<IRExpression>),
    Scope {
        is_unsafe: bool,
        body: Vec<IRNode>
    }
    // DefineVariable {
    //     name: String,
    //     data_type: Type,
    //     expression: Option<Expression>
    // }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<IRNode>
}

#[allow(unused)]
pub struct Variables {
    pub variables: HashMap<&'static String, Type>,
}
#[allow(unused)]
impl Variables {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
