use super::Program;
use crate::{
    parser::{ASTNode, Node, Path, Type},
    AnalyzeResult, CompileError,
};
use std::{collections::HashMap, path::PathBuf};

pub type Function = (Vec<(String, Type)>, Option<Type>);
pub type FunctionTypes = HashMap<Path, Function>;

pub fn analyze(modules: HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<Program> {
    let types = parse_functions(&modules)?;


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

    return Ok(functions);
}

pub fn recursive_function(path: &PathBuf, nodes: Vec<ASTNode>) {

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
