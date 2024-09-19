use std::collections::HashMap;

use crate::parser::{Expression, Type};




#[allow(unused)]
#[derive(Debug)]
pub enum IRNode { 
    Expression(Expression),
    DefineVariable {
        name: String,
        data_type: Type,
        expression: Option<Expression>
    }
}

#[derive(Debug)]
pub struct Function {
    pub body: Vec<IRNode>
}

#[allow(unused)]
#[derive(Debug)]
pub struct Module {
    pub functions: HashMap<String, Vec<Function>>
}