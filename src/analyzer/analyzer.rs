use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Node, Program}, BuildError
};

use super::info::{Function, IRNode};

pub fn analyze(program: Program) -> Result<Program, BuildError> {
    println!("{:#?}", program);

    for (path, module) in program.modules.iter() {
        #[allow(unused)]
        match analyze_module(&program, &path, &module.body) {
            Ok(nodes) => {
                // let new_module = Module { body: nodes };
                // new_program
                //     .modules
                //     .insert(path.clone(), new_module)
                //     .unwrap();
            }
            Err(error) => return Err(error),
        }
    }

    todo!()
}

#[allow(unused)]
fn analyze_module(
    program: &Program,
    relative_path: &PathBuf,
    body: &Vec<ASTNode>,
) -> Result<Vec<Function>, BuildError> {
    let mut tree: Vec<Function> = Vec::new();

    for node in body {
        match &node.node {
            #[allow(unused)]
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => {
                let nodes = match analyze_body(body) {
                    Ok(b) => b,
                    Err(error) => return Err(error),
                };

                tree.push(Function { body: nodes });
            }
            Node::Import(_, _) => {}
            _ => panic!("Expected function"),
        }
    }
 
    return Ok(tree);
}

#[allow(unused)]
fn analyze_body(nodes: &Vec<ASTNode>) -> Result<Vec<IRNode>, BuildError> {
    todo!()
}
