use std::{collections::HashMap, path::PathBuf};

use crate::{
    parser::{ASTNode, Node, Path},
    AnalyzeResult, CompileError,
};

use super::{CustomType, IREnum, IRFunction, IRStruct};

pub fn parse_types(modules: &HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<Types> {
    let mut types = Types::new();
    parse_functions(&mut types, &modules)?;
    return Ok(types);
}

pub fn parse_functions(
    types: &mut Types,
    modules: &HashMap<PathBuf, Vec<ASTNode>>,
) -> AnalyzeResult<()> {
    let mut bodies: Vec<(Path, &Vec<ASTNode>)> = Vec::new();

    for (pathbuf, body) in modules {
        for ast in body {
            handle_node(pathbuf, types, &mut bodies, ast)?;
        }
    }

    for (path, body) in bodies {
        recursive_function(&path, &body)?;
    }

    return Ok(());
}

fn handle_node(
    pathbuf: &PathBuf,
    types: &mut Types,
    bodies: &mut Vec<(Path, &Vec<ASTNode>)>,
    ast: &ASTNode,
) -> AnalyzeResult<()> {
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
            let mut path = convert_pathbuf(&pathbuf);
            path.add(name.clone());

            let mut function = IRFunction {
                generics: None,
                parameters: parameters.to_owned(),
                return_type: return_type.to_owned(),
            };

            if generics.len() > 0 {
                function.generics = Some(generics.to_owned());
                if types
                    .generic_functions
                    .insert(path.clone(), function)
                    .is_some()
                {
                    return Err(CompileError::new(
                        format!("{} is already defined", name),
                        ast.lines.start,
                    ));
                };
            } else {
                if types.functions.insert(path.clone(), function).is_some() {
                    return Err(CompileError::new(
                        format!("{} is already defined", name),
                        ast.lines.start,
                    ));
                };
            }
        }
        Node::Import(_) => {}
        Node::Struct {
            export,
            name,
            generics,
            body,
        } => {
            let mut path = convert_pathbuf(&pathbuf);
            path.add(name.clone());

            let custom_struct = IRStruct {
                generics: generics.clone(),
                name: name.clone(),
                fields: Vec::new(),
            };

            types.custom.insert(path, CustomType::Struct(custom_struct));
        }
        Node::Enum {
            export,
            name,
            generics,
            body,
        } => {
            let mut path = convert_pathbuf(&pathbuf);
            path.add(name.clone());

            let custom_struct = IREnum {
                generics: generics.clone(),
                name: name.clone(),
                enums: body.clone()
            };

            types.custom.insert(path, CustomType::Enum(custom_struct));
        }
        _ => {
            return Err(CompileError::new(
                format!("Function expected"),
                ast.lines.start,
            ))
        }
    }

    return Ok(());
}

fn recursive_function(module_path: &Path, nodes: &Vec<ASTNode>) -> AnalyzeResult<()> {
    for ast in nodes {
        match &ast.node {
            Node::Call(local_path, arguments) => {
                let mut call_path = module_path.clone();
                call_path.push(local_path);
            }
            Node::Scope { is_unsafe, body } => recursive_function(module_path, body)?,
            _ => continue,
        }
    }

    return Ok(());
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


#[derive(Debug, Default)]
pub struct Types {
    pub custom: HashMap<Path, CustomType>,
    pub generic_custom: HashMap<Path, CustomType>,

    pub generic_functions: HashMap<Path, IRFunction>,
    pub functions: HashMap<Path, IRFunction>,
}
impl Types {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn get_type(&self, path: &Path) -> AnalyzeResult<&CustomType> {
        return match self.custom.get(path) {
            Some(t) => Ok(t),
            None => todo!(),
        };
    }
}
