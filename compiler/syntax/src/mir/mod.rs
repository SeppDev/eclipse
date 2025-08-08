mod expression;
mod module;
mod node;
mod types;

pub use expression::*;
pub use module::*;
pub use node::*;
pub use types::*;

#[derive(Debug)]
pub struct Parameter {
    pub pointer: bool,
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Node>,
}
