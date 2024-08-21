use crate::{builder::function::Function, parser::Node};

use super::Program;

pub fn scope(nodes: Vec<Node>, function: &mut Function, program: &mut Program) {
    for node in nodes {
        match node {
            #[allow(unused)]
            Node::DefineVariable {
                name,
                mutable,
                var_type,
                expression,
            } => function.define_variable(&name, var_type.unwrap(), expression.unwrap()),
            _ => todo!("{:#?}", node),
        }
    }
}
