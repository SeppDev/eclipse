mod node;
mod types;

pub use node::*;
pub use types::*;

pub struct Parameter {
    pointer: bool,
    name: String,
    data_type: Type,
}
