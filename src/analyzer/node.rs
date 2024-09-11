use std::collections::HashMap;

use crate::parser::Type;

#[derive(Debug)]
pub enum IRNode {}

#[derive(Debug)]
pub struct IRFunction {
    pub stack_size: usize,
    pub body: Vec<IRNode>,
}

#[derive(Debug, Default)]
pub struct IRProgram {
    // pub types: HashMap<String, >
    pub functions: HashMap<String, (Vec<(String, Type)>, Option<Type>)>,
    pub body: HashMap<String, IRFunction>,
}

impl IRProgram {
    pub fn new() -> Self {
        Self::default()
    }
}
