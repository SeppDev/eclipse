use super::{Parameter, Type};

#[derive(Debug)]
pub enum Node {
    DeclareVariable {
        name: String,
        mutable: bool,
        data_type: Type,
        value: Box<Node>,
    },
    Block(Vec<Node>),
    Return(Option<Box<Node>>),
    Integer(String),
    Boolean(bool),
}
