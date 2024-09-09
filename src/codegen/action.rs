

// use crate::compiler::Type;

// use super::variables::Variables;

use crate::parser::Type;

#[derive(Debug)]
pub enum Operation {
    Read(usize),
    Integer(isize),
    UInteger(usize),
}

#[derive(Debug)]
pub enum Action {
    // LoadVariable(usize),\

    StoreVariable(usize, Operation),
    Call(String, Vec<Operation>),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub stack_size: usize,
    pub actions: Vec<Action>,
    // pub parameters: Vec<(String, Type)>,
    // pub variables: Variables,
}
impl Function {
    pub fn new(name: String, _parameters: Vec<(String, Type)>) -> Self {
        Self {
            // parameters: parameters,
            // variables: Variables::new(),
            name: name,
            actions: Vec::new(),
            stack_size: 0,
        }
    }
}
