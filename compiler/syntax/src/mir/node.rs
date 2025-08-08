use super::{Expression, Parameter, Type};

#[derive(Debug)]
pub enum Node {
    Set {
        name: String,
        data_type: Type,
        value: Expression,
    },
    Goto(String),
    Allocate(Type),
    Return(Option<Expression>),
}
