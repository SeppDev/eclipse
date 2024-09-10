use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Node, Program},
    CompileError,
};

use super::node::IRProgram;

pub fn analyze(program: Program) -> Result<IRProgram, CompileError> {
    let mut IRprogram = IRProgram::new();

    for (path, mut module) in program.modules {
        match parse_root(&mut module.body, path) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
    }

    return Ok(IRprogram)
}

pub fn parse_root(nodes: &mut Vec<ASTNode>, relative_path: PathBuf) -> Result<(), CompileError> {
    for ast_node in nodes {
        let node = &ast_node.node;
        match node {
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => {}
            Node::Import(_, _) => continue,
            _ => panic!("Expected function"), // _ => Err(CompileError::BuildProblem(BuildProblem::new(BuildError::, relative_path, line)))
        }
    }

    return Ok(());
}
