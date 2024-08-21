use std::{collections::HashMap, path::PathBuf};

use crate::{parser::{parse, Node, Type}, FILE_EXTENSION};
use eclipse::{BuildError, CompileError};
use scope::scope;

mod call;
mod expression_type;
mod scope;

struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}

struct Variable {
    pub mutable: bool,
    pub var_type: Type,
}

#[derive(Default)]
struct Scope {
    pub variables: HashMap<String, Variable>,
}


pub fn analyze(nodes: Vec<Node>, path: PathBuf, _is_main: bool) -> Result<Vec<Node>, CompileError> {
    let mut tree: Vec<Node> = Vec::new();
    
    let mut functions: HashMap<String, Function> = HashMap::new();
    for node in nodes.clone() {
        match node {
            Node::Module(module) => {
                if module == "main" {
                    return Err(CompileError::Building(BuildError::ModuleNotFound))
                }

                let parent = match path.parent() {
                    Some(path) => path.to_path_buf().join(format!("{}.{}", module, FILE_EXTENSION)),
                    None => return Err(CompileError::Building(BuildError::ModuleNotFound))
                };

                let nodes = match parse(&parent) {
                    Ok(nodes) => nodes,
                    Err(error) => return Err(error)
                };

                for node in nodes {
                    tree.push(node);
                }
            }
            #[allow(unused)]
            Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => {
                let function = Function {
                    parameters: parameters.clone(),
                    return_type: return_type,
                };

                let mut params = HashMap::new();
                for (name, t) in parameters {
                    match params.insert(name.clone(), t) {
                        Some(_) => return Err(CompileError::Building(BuildError::AlreadyDefined(name))),
                        None => continue,
                    }
                }

                match functions.insert(name.clone(), function) {
                    Some(_) => return Err(CompileError::Building(BuildError::AlreadyDefined(name))),
                    None => continue,
                };
            }
            _ => continue,
        }
    }

    for node in nodes {
        match node {
            #[allow(unused)]
            Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => {
                let mut scope_parameters = Scope::default();
                for (name, t) in parameters.clone() {
                    scope_parameters.variables.insert(
                        name,
                        Variable {
                            mutable: false,
                            var_type: t,
                        },
                    );
                }

                let body = match scope(
                    body,
                    &mut scope_parameters,
                    &functions.get(&name).unwrap(),
                    &functions,
                ) {
                    Ok(nodes) => nodes,
                    Err(error) => return Err(CompileError::Building(error)),
                };
                match return_type.clone() {
                    Some(t) => {
                        match body.last() {
                            Some(t) => match t {
                                Node::Return(_) => {},
                                _ => return Err(CompileError::Building(BuildError::WrongReturnType))
                            },
                            None => return Err(CompileError::Building(BuildError::NoNodeFound))
                        }
                    },
                    None => {}
                };

                tree.push(Node::Function {
                    public,
                    name: name.clone(),
                    return_type: return_type,
                    parameters: parameters,
                    body: body,
                });
            }
            Node::Module(_) => continue,
            _ => return Err(CompileError::Building(BuildError::NoNodeFound)),
        }
    }

    return Ok(tree)
}
