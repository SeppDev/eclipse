use super::{FunctionTypes, Program};
use crate::{
    analyzer::Types, parser::{ASTNode, Node, Path, Type}, AnalyzeResult, CompileError
};
use std::{collections::HashMap, path::PathBuf};

pub fn analyze(modules: HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<Program> {
    println!("{:#?}", modules);

    // let types = parse_functions(&modules)?;
    let types = Types::new();

    println!("{:#?}", types);
    todo!();
}

pub fn parse_functions(modules: &HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<FunctionTypes> {
    let mut functions: FunctionTypes = HashMap::new();

    for (path, body) in modules {
        for ast in body {
            match &ast.node {
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

                    let mut path = convert_pathbuf(path);
                    path.add(name.clone());
                    functions.insert(path, (parameters.clone(), return_type.clone()));
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

    for (path, body) in modules {
        for ast in body {
            match &ast.node {
                Node::Function {
                    export,
                    is_unsafe,
                    name,
                    generics,
                    parameters,
                    return_type,
                    body,
                } => recursive_function(&convert_pathbuf(&path), &body),
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

    return Ok(functions);
}

pub fn recursive_function(module_path: &Path, nodes: &Vec<ASTNode>) {
    for ast in nodes {
        match &ast.node {
            Node::Call(local_path, arguments) => {
                let mut call_path = module_path.clone();
                call_path.push(local_path);
            }
            Node::Scope { is_unsafe, body } => {
                recursive_function(module_path, body);
            }
            _ => continue,
        }
    }
}

pub fn convert_pathbuf(pathbuf: &PathBuf) -> Path {
    let mut components = pathbuf.components();
    let mut path = Path::new(String::from(
        components.next().unwrap().as_os_str().to_str().unwrap(),
    ));

    loop {
        let cmp = match components.next() {
            Some(a) => a,
            None => break,
        };
        path.add(String::from(cmp.as_os_str().to_str().unwrap()));
    }

    return path;
}
