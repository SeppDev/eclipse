use std::collections::HashMap;

use crate::{parser::{Type, Value}, Path};

use super::ModuleTypes;

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(String),
    Call(Path, Vec<IRExpression>)
}

#[derive(Debug)]
pub enum IRNode {
    // Scope {
    //     is_unsafe: bool,
    //     body: Vec<IRNode>,
    // },
    Return(Option<IRExpression>),
    DefineVariable {
        name: String,
        data_type: Type,
        expression: IRExpression,
    },
}

#[derive(Debug)]
pub struct IRFunction {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub nodes: Vec<IRNode>,
}

#[derive(Debug)]
pub struct IRModule {
    pub functions: HashMap<String, IRFunction>
}

#[derive(Debug)]
pub struct IRProgram {
    pub modules: HashMap<Path, IRModule>,
    pub types: ModuleTypes,
}
impl IRProgram {
    pub fn get_function(&self, path: Path) {

    }
    pub fn get_type(&self, path: Path) {

    }
}