use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Module, Node, Program},
    CompileError,
};

pub fn analyze(program: Program) -> Result<Program, CompileError> {
    let mut new_program = Program::new(program.project_path.clone());

    for (path, module) in program.modules.iter() {
        match analyze_module(&program, &path, &module.body) {
            Ok(nodes) => {
                let new_module = Module { body: nodes };
                new_program
                    .modules
                    .insert(path.clone(), new_module)
                    .unwrap();
            }
            Err(error) => return Err(error),
        }
    }

    todo!()
}

fn analyze_module(
    program: &Program,
    relative_path: &PathBuf,
    body: &Vec<ASTNode>,
) -> Result<Vec<Node>, CompileError> {
    let mut tree: Vec<Node> = Vec::new();

    for node in body {
        match &node.node {
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => {
                let new = match analyze_body(body) {
                    Ok(b) => b,
                    Err(error) => return Err(error),
                };
                tree.push(Node::Function {
                    export: export.to_owned(),
                    is_unsafe: is_unsafe.to_owned(),
                    name: name.to_owned(),
                    parameters: parameters.to_owned(),
                    return_type: return_type.to_owned(),
                    body: new,
                });
            }
            Node::Import(_, _) => {}
            _ => panic!("Expected function"),
        }
    }

    return Ok(tree);
}

fn analyze_body(nodes: &Vec<ASTNode>) -> Result<Vec<Node>, CompileError> {
    todo!()
}
