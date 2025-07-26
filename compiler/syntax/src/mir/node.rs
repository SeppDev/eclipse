use super::{Expression, Parameter, Type};

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Node>,
    },
    DeclareVariable {
        name: String,
        data_type: Type,
        value: Expression,
    },
    Block(Vec<Node>),
    Allocate(Type),
    Return(Option<Expression>),
}
