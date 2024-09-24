use super::Program;
use crate::{
    parser::{ASTNode, Node},
    AnalyzeResult, CompileError,
};
use std::{collections::HashMap, path::PathBuf};

pub fn analyze(modules: HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<Program> {
    for (path, body) in modules {
        for ast in body {
            match ast.node {
                Node::Function {
                    export,
                    is_unsafe,
                    name,
                    generics,
                    parameters,
                    return_type,
                    body,
                } => {
                    if generics.len() > 0 {
                        continue;
                    }
                    println!("{:?}", name)
                }
                Node::Import(_) => continue,
                _ => {
                    return Err(CompileError::new(
                        format!("Function expected"),
                        ast.lines.start,
                    ))
                }
            }
        }
    }
    todo!()
}
