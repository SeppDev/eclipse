use super::{Parameter, Type};

pub enum Node {
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Box<Node>,
    },
    DeclareVariable {
        name: String,
        data_type: Type,
        value: Box<Node>,
    },
    Block(Vec<Node>),
}
