use crate::compiler::types::Type;

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Node>,
    },
    Variable {
        name: String,
        value: Expression
    },
    Return(Option<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Value(Value)
}

#[derive(Debug)]
pub enum Value {
    Integer { minus: bool, integer: String },
}
