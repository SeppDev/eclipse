use std::{collections::HashMap, path::PathBuf};

use crate::parser::{Expression, Node, Type, Value};
use eclipse::{BuildError, CompileError};

use super::program::{self, Program};

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

fn call(
    function: &Function,
    arguments: Vec<Expression>,
    scope: &Scope,
    functions: &HashMap<String, Function>,
) -> Option<BuildError> {
    if arguments.len() != function.parameters.len() {
        return Some(BuildError::TooFewOrManyArguments);
    }

    for (index, argument) in arguments.into_iter().enumerate() {
        let (_, t) = function.parameters.get(index).unwrap();
        let a = match expression_type(argument, scope, function, functions) {
            Ok(a) => a,
            Err(error) => return Some(error),
        };
        if a != t.to_owned() {
            return Some(BuildError::WrongType);
        }
    }

    None
}

fn expression_type(
    expression: Expression,
    scope: &Scope,
    function: &Function,
    functions: &HashMap<String, Function>,
) -> Result<Type, BuildError> {
    use crate::parser::Integer;
    use crate::parser::Type;

    let t = match expression {
        Expression::Value(value, _) => match value {
            Value::Boolean(_) => Type::Boolean,
            Value::String(_) => Type::String,
            Value::Integer(_) => Type::Integer(Integer::i64),
        },
        Expression::GetVariable(name) => {
            let variable = match scope.variables.get(&name) {
                Some(a) => a,
                None => return Err(BuildError::NotDefined(name)),
            };
            variable.var_type.clone()
        }
        Expression::Call(name, arguments) => match functions.get(&name) {
            Some(function) => {
                match call(function, arguments, scope, functions) {
                    Some(error) => return Err(error),
                    None => {}
                };
                match function.return_type.clone() {
                    Some(t) => t,
                    None => return Err(BuildError::WrongReturnType),
                }
            }
            None => return Err(BuildError::NotDefined(name)),
        },
        Expression::BinaryOperation(a, _, b) => {
            let a = match expression_type(*a, scope, function, functions) {
                Ok(a) => a,
                Err(error) => return Err(error),
            };
            let b = match expression_type(*b, scope, function, functions) {
                Ok(a) => a,
                Err(error) => return Err(error),
            };

            if a != b {
                return Err(BuildError::WrongReturnType);
            }

            return Ok(a);
        }
    };

    return Ok(t);
}

fn scope(
    nodes: Vec<Node>,
    scope: &mut Scope,
    function: &Function,
    functions: &HashMap<String, Function>,
) -> Result<Vec<Node>, BuildError> {
    let mut tree: Vec<Node> = Vec::new();
    let mut declared: Vec<String> = Vec::new();

    for node in nodes {
        match node {
            Node::Call(name, arguments) => {
                let function = match functions.get(&name) {
                    Some(func) => func,
                    None => return Err(BuildError::NotDefined(name)),
                };

                match call(function, arguments.clone(), &scope, &functions) {
                    Some(error) => return Err(error),
                    None => {},
                }

                tree.push(Node::Call(name, arguments));
            }
            Node::DefineVariable {
                name,
                mutable,
                var_type,
                expression,
            } => {
                let expression = match expression {
                    Some(expression) => expression,
                    None => {
                        let var_type = match var_type {
                            Some(t) => t,
                            None => return Err(BuildError::WrongType),
                        };

                        tree.push(Node::DefineVariable {
                            name: name.clone(),
                            mutable: mutable,
                            var_type: Some(var_type.clone()),
                            expression: expression,
                        });

                        declared.push(name.clone());
                        match scope.variables.insert(
                            name.clone(),
                            Variable {
                                mutable: mutable,
                                var_type: var_type,
                            },
                        ) {
                            Some(_) => return Err(BuildError::AlreadyDefined(name)),
                            None => continue,
                        };
                    }
                };

                let found_type =
                    match expression_type(expression.clone(), scope, function, functions) {
                        Ok(t) => t,
                        Err(error) => return Err(error),
                    };

                let var_type: Type = match var_type {
                    Some(t) => {
                        let mut old = t;
                        if old != found_type {
                            old = match old {
                                Type::Integer(_) => match found_type {
                                    Type::Integer(_) => old,
                                    _ => return Err(BuildError::WrongType),
                                },
                                _ => return Err(BuildError::WrongType),
                            };
                        }

                        old
                    }
                    None => found_type,
                };

                tree.push(Node::DefineVariable {
                    name: name.clone(),
                    mutable: mutable,
                    var_type: Some(var_type.clone()),
                    expression: Some(expression),
                });

                match scope.variables.insert(
                    name.clone(),
                    Variable {
                        mutable: mutable,
                        var_type: var_type,
                    },
                ) {
                    Some(_) => return Err(BuildError::AlreadyDefined(name)),
                    None => continue,
                };
            }
            Node::SetVariable {
                name,
                mut expression,
            } => {
                let variable = match scope.variables.get(&name) {
                    Some(variable) => variable,
                    None => return Err(BuildError::NotDefined(name)),
                };
                if variable.mutable == false {
                    return Err(BuildError::NotMutable(name));
                }
                let found_type =
                    match expression_type(expression.clone(), scope, function, functions) {
                        Ok(t) => t,
                        Err(error) => return Err(error),
                    };
                if found_type != variable.var_type.clone() {
                    match variable.var_type {
                        Type::Integer(_) => {
                            let value: isize = match expression {
                                Expression::Value(v, _) => match v {
                                    Value::Integer(v) => v,
                                    _ => return Err(BuildError::WrongMutableType(name)),
                                },
                                _ => return Err(BuildError::WrongMutableType(name)),
                            };

                            expression =
                                Expression::Value(Value::Integer(value), variable.var_type.clone())
                        }
                        _ => return Err(BuildError::WrongMutableType(name)),
                    }
                }

                tree.push(Node::SetVariable {
                    name: name,
                    expression: expression,
                })
            }
            Node::Return(expression) => {
                let return_type = match function.return_type.clone() {
                    Some(t) => t,
                    None => {
                        match expression {
                            Some(_) => return Err(BuildError::WrongReturnType),
                            None => tree.push(Node::Return(None)),
                        }
                        break;
                    }
                };
                let expression = match expression {
                    Some(e) => e,
                    None => return Err(BuildError::WrongReturnType),
                };
                let found_type =
                    match expression_type(expression.clone(), scope, function, functions) {
                        Ok(a) => a,
                        Err(error) => return Err(error),
                    };

                if return_type != found_type {
                    // panic!("{:#?} != {:#?}", return_type, found_type)
                    return Err(BuildError::WrongReturnType)
                }
                tree.push(Node::Return(Some(expression)));
                break;
            }
            _ => continue,
        }
    }

    for key in declared {
        scope.variables.remove(&key);
    }

    return Ok(tree);
}

pub fn analyze(nodes: Vec<Node>, path: PathBuf) -> Result<Vec<Node>, CompileError> {
    todo!()
}

fn _analyze(nodes: Vec<Node>, path: PathBuf) -> Result<Vec<Node>, BuildError> {
    let mut tree = Vec::new();

    let mut functions: HashMap<String, Function> = HashMap::new();
    for node in nodes.clone() {
        match node {
            Node::Module(module) => {
                println!("{:?}, {:?}", path.parent(), module);
            }
            #[allow(unused)]
            Node::Function {
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
                        Some(_) => return Err(BuildError::AlreadyDefined(name)),
                        None => continue,
                    }
                }

                match functions.insert(name.clone(), function) {
                    Some(_) => return Err(BuildError::AlreadyDefined(name)),
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
                    Err(error) => return Err(error),
                };
                match return_type.clone() {
                    Some(t) => {
                        match body.last() {
                            Some(t) => match t {
                                Node::Return(_) => {},
                                _ => return Err(BuildError::Unkown)
                            },
                            None => return Err(BuildError::Unkown)
                        }
                    },
                    None => {}
                };

                tree.push(Node::Function {
                    name: name.clone(),
                    return_type: return_type,
                    parameters: parameters,
                    body: body,
                });
            }
            _ => return Err(BuildError::Unkown),
        }
    }

    return Ok(tree);
}
