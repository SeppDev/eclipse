use crate::{
    builder::{variables::Variables, writer::Writer},
    parser::Node,
};

pub struct Function {
    stack_size: usize,
    variables: Variables,
}
impl Function {
    pub fn new() -> Self {
        Self {
            stack_size: 8,
            variables: Variables::new(),
        }
    }
}

pub fn scope(nodes: Vec<Node>, writer: &mut Writer, Function: &mut Function) {
    for node in nodes {
        match node {
            Node::DefineVariable {
                name,
                mutable,
                var_type,
                expression,
            } => {}
            _ => todo!("{:#?}", node),
        }
    }
}
