use crate::{
    parser::{ASTNode, Modules, Node, Path},
    AnalyzeResult,
};

use super::structures::Types;

pub fn parse_datastructures(modules: &Modules) -> AnalyzeResult<Types> {
    let mut types = Types::new();

    for (path, nodes) in modules {
        parse_module(&mut types, path, nodes)?
    }

    return Ok(types);
}

fn parse_module(
    types: &mut Types,
    relative_path: &Path,
    nodes: &Vec<ASTNode>,
) -> AnalyzeResult<()> {
    for ast in nodes {
        match &ast.node {
            Node::Enum {
                export,
                name,
                generics,
                body,
            } => {
                
            }
            Node::Struct {
                export,
                name,
                generics,
                body,
            } => {}
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                types.
            }
            _ => continue,
        }
    }

    return Ok(());
}
