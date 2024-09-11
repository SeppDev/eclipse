use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Node, Program},
    CompileError, FILE_EXTENSION,
};

use super::{node::IRProgram, IRFunction, IRNode};

fn create_function_path(relative_path: &PathBuf, name: String) -> String {
    let mut function_path = relative_path
        .to_str()
        .unwrap()
        .replace(format!(".{}", FILE_EXTENSION).as_str(), "")
        .replace("/", ".");

    function_path.push_str(format!(".{}", name).as_str());

    return function_path;
}

pub fn analyze(program: Program) -> Result<IRProgram, CompileError> {
    let mut ir_program = IRProgram::new();

    for (path, module) in program.modules {
        for node in module.body {
            match node.node {
                #[allow(unused)]
                Node::Function {
                    export,
                    is_unsafe,
                    name,
                    parameters,
                    return_type,
                    body,
                } => {
                    ir_program.functions.insert(
                        create_function_path(&path, name),
                        (parameters.clone(), return_type.clone()),
                    );
                }
                Node::Import(_, _) => continue,
                node => panic!("Expected function got: {:?}", node),
            }
        }
    }

    return Ok(ir_program);
}

pub fn parse_root(
    nodes: Vec<ASTNode>,
    relative_path: PathBuf,
) -> Result<Vec<IRFunction>, CompileError> {
    let mut functions = Vec::new();

    for ast_node in nodes {
        let node = ast_node.node;
        match node {
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => match parse_body(body, is_unsafe.to_owned()) {
                Ok(nodes) => functions.push(IRFunction {
                    body: nodes,
                    stack_size: 16,
                }),
                Err(error) => return Err(error),
            }, //functions.push(IRFunction {
            // path: format!("{}.{}", function_path, name),,
            Node::Import(_, _) => continue,
            _ => panic!("Expected function"), // _ => Err(CompileError::BuildProblem(BuildProblem::new(BuildError::, relative_path, line)))
        }
    }

    return Ok(functions);
}

pub fn parse_body(body: Vec<ASTNode>, is_unsafe: bool) -> Result<Vec<IRNode>, CompileError> {
    let mut tree = Vec::new();

    for node in body {
        match node.node {
            Node::Scope { is_unsafe, body } => match parse_body(body, is_unsafe) {
                Ok(body) => {}
                Err(error) => return Err(error),
            },
            Node::DefineVariable {
                mutable,
                name,
                var_type,
                expression,
            } => {}
            _ => todo!(),
        }
    }

    return Ok(tree);
}
