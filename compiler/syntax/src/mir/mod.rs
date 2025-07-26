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
    pointer: bool,
    name: String,
    data_type: Type,
}
