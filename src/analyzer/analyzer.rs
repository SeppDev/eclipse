use std::collections::HashMap;

use crate::{
    analyzer::parse_datastructures,
    parser::{Modules, Node, Path},
    AnalyzeResult,
};

use super::{program::Function, Types};

pub fn analyze(modules: Modules) -> AnalyzeResult<()> {
    println!("{:#?}", modules);
    let types = parse_datastructures(&modules)?;
    println!("{:#?}", types);
    let functions = parse(&types, modules)?;
    println!("{:#?}", functions);

    todo!()
}

fn parse(types: &Types, modules: Modules) -> AnalyzeResult<HashMap<Path, Function>> {
    let mut functions = HashMap::new();
    for (relative_path, nodes) in modules {
        for ast in nodes {
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
                    let function = parse_function(types)?;
                    let mut path = relative_path.clone();
                    path.add(name);
                    functions.insert(path, function);
                }
                _ => continue,
            }
        }
    }

    return Ok(functions);
}

fn parse_function(types: &Types) -> AnalyzeResult<Function> {
    return Ok(Function {});
}
