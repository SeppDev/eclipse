use crate::parser::{Type, Value};

#[derive(Debug)]
pub struct Program {
    
}

#[derive(Debug)]
pub enum IRExpression {
    Value(Value)
}


#[derive(Debug)]
pub enum IRNode {
    Return(Option<IRExpression>)
    // DefineVariable {
    //     name: String,
    //     data_type: Type,
    //     expression: Option<Expression>
    // }
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Type,
    pub body: Vec<IRNode>,
}
