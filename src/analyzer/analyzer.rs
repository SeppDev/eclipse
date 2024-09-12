use std::path::PathBuf;

use crate::{parser::{ASTNode, Module, Node, Program}, CompileError};

pub fn analyze(program: Program) -> Result<Program, CompileError> {
    let mut new_program = Program::new(program.project_path);

    for (path, module) in &program.modules {
        match analyze_module(&program, path, module) {
            Ok(nodes) => {
                let new_module = Module {
                    body: nodes
                };
                new_program.modules.insert(path.clone(), new_module).unwrap();
            },
            Err(error) => return Err(error)
        }
    }   

    todo!()
}

fn analyze_module(program: &Program, relative_path: &PathBuf, module: &Module) -> Result<Vec<ASTNode>, CompileError> {
    let mut tree = Vec::new();

    return Ok(tree)
}

fn analyze_body() -> Result<Vec<ASTNode>, CompileError> {

    todo!()
}