use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Node, Program},
    CompileError,
};

use super::{node::IRProgram, IRFunction};

pub fn analyze(program: Program) -> Result<IRProgram, CompileError> {
    let mut ir_program = IRProgram::new();

    for (path, mut module) in program.modules {
        match parse_root(&mut module.body, path) {
            Ok(functions) => ir_program.push_functions(functions),
            Err(error) => return Err(error),
        };
    }

    return Ok(ir_program);
}

pub fn parse_root(
    nodes: &mut Vec<ASTNode>,
    relative_path: PathBuf,
) -> Result<Vec<IRFunction>, CompileError> {
    let mut functions = Vec::new();

    let function_path = relative_path.to_str().unwrap().replace(".eclipse", "").replace("/", ".");


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
            } => functions.push(IRFunction {
                path: format!("{}.{}", function_path, name),
            }),
            Node::Import(_, _) => continue,
            _ => panic!("Expected function"), // _ => Err(CompileError::BuildProblem(BuildProblem::new(BuildError::, relative_path, line)))
        }
    }

    return Ok(functions);
}

pub fn parse_function() -> Result<(), CompileError> {
    Ok(())
}
