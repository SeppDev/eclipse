use std::collections::HashMap;

use eclipse::BuildError;

use crate::parser::{Expression, Node, Type, Value};

use super::{Function, Scope};


pub fn scope(
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
                    None => {}
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
                    return Err(BuildError::WrongReturnType);
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
