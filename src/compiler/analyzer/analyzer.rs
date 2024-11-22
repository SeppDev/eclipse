use crate::compiler::{
    counter::NameCounter,
    errors::{CompileMessages, CompileResult, Location, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

use super::{
    node::{IRExpression, IRExpressionInfo, IRFunction, IRNode},
    parse_types,
    variables::Variables,
    FileTypes, IRProgram, IRType,
};

pub fn analyze(
    program: ParsedProgram,
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
) -> CompileResult<IRProgram> {
    let mut functions = Vec::new();
    let types = parse_types(compile_messages, name_counter, &program)?;
    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    analyze_file(compile_messages, &mut functions, &types, program.main)?;

    return Ok(IRProgram { functions });
}

fn analyze_file(
    compile_messages: &mut CompileMessages,
    functions: &mut Vec<IRFunction>,
    types: &FileTypes,
    file: ParsedFile,
) -> CompileResult<()> {
    for (_, file) in file.imports {
        analyze_file(compile_messages, functions, types, file)?;
    }

    for info in file.body {
        let (_, _, key, parameters, return_type, body) = match info.node {
            Node::Function {
                export,
                name,
                key,
                parameters,
                return_type,
                body,
            } => (export, name, key, parameters, return_type, body),
            _ => continue,
        };

        let mut variables = Variables::new();
        variables.create_state();
        for (key, t) in &parameters {
            let result = variables
                .insert(&key, false, t.clone(), info.location.clone())
                .1;

            match result {
                Ok(()) => {}
                Err(var) => {
                    compile_messages.create(
                        MessageKind::Error,
                        var.location.clone(),
                        file.relative_file_path.clone(),
                        format!("Duplicate parameter '{}'", key),
                        "",
                    );
                }
            }
        }

        let mut nodes = Vec::new();
        analyze_body(
            compile_messages,
            types,
            &file.relative_file_path,
            &mut variables,
            &Some(return_type.clone()),
            body,
            &mut nodes,
        )?;

        if return_type.is_void() {
            let last = nodes.last();
            match last {
                Some(node) => match node {
                    IRNode::Return(_) => {}
                    _ => nodes.push(IRNode::Return(IRExpressionInfo::from(
                        IRExpression::Void,
                        &Type::void(),
                    ))),
                },
                None => nodes.push(IRNode::Return(IRExpressionInfo::from(
                    IRExpression::Void,
                    &Type::void(),
                ))),
            }
        } else {
            nodes.last().is_none().then(|| {
                compile_messages.create(
                    MessageKind::Error,
                    info.location.clone(),
                    file.relative_file_path.clone(),
                    format!("Expected return"),
                    "",
                );
            });
        }

        functions.push(IRFunction {
            name: key,
            parameters,
            return_type,
            body: nodes,
        })
    }

    return Ok(());
}

fn analyze_body(
    compile_messages: &mut CompileMessages,
    types: &FileTypes,
    relative_file_path: &Path,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<NodeInfo>,
    nodes: &mut Vec<IRNode>,
) -> CompileResult<()> {
    use super::super::parser::Node;
    variables.create_state();

    let mut body = body.into_iter();

    loop {
        let info = match body.next() {
            Some(info) => info,
            None => break,
        };

        let ir_node: IRNode = match info.node {
            Node::Call(path, arguments) => {
                let function = match types.get_function(relative_file_path, &path)? {
                    Some(f) => f,
                    None => {
                        compile_messages.create(
                            MessageKind::Error,
                            info.location.clone(),
                            relative_file_path.clone(),
                            format!("Failed to find function: {}", path.components().join("::")),
                            "",
                        );
                        continue;
                    }
                };
                IRNode::Call(function.name.clone(), Vec::new())
            }
            Node::Scope(body) => {
                analyze_body(
                    compile_messages,
                    types,
                    relative_file_path,
                    variables,
                    return_type,
                    body,
                    nodes,
                )?;
                continue;
            }
            Node::Return(expression) => {
                let (expression, _) = analyze_expression(
                    compile_messages,
                    relative_file_path,
                    types,
                    variables,
                    return_type,
                    &info.location,
                    expression,
                )?;
                nodes.push(IRNode::Return(expression));
                break;
            }
            Node::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => {
                if expression.is_none() {
                    todo!("Expression required");
                }
                let (expression, data_type) = analyze_expression(
                    compile_messages,
                    relative_file_path,
                    types,
                    variables,
                    &data_type,
                    &info.location,
                    expression,
                )?;

                let (current, result) = variables.insert(
                    &name,
                    mutable.clone(),
                    data_type.clone(),
                    info.location.clone(),
                );

                match result {
                    Ok(_) => {}
                    Err(old) => {
                        let message = compile_messages.create(
                            MessageKind::Error,
                            old.location.clone(),
                            relative_file_path.clone(),
                            format!("'{}' is already declared", name.clone()),
                            "",
                        );
                        message.push("", info.location.clone());
                    }
                }

                IRNode::DeclareVariable(current.name.clone(), expression)
            }
            Node::SetVariable { name, expression } => {
                let variable = match variables.get(&name, true) {
                    Some(var) => var.clone(),
                    None => todo!(),
                };
                if !variable.mutable {
                    compile_messages.create(
                        MessageKind::Error,
                        info.location.clone(),
                        relative_file_path.clone(),
                        format!("Cannot asign to immutable variable '{}'", name.clone()),
                        "",
                    );
                }
                let (expression, _data_type) = analyze_expression(
                    compile_messages,
                    relative_file_path,
                    types,
                    variables,
                    &Some(variable.data_type),
                    &info.location,
                    expression,
                )?;

                IRNode::SetVariable(variable.name.clone(), expression)
            }
            _ => {
                compile_messages.create(
                    MessageKind::Error,
                    info.location.clone(),
                    relative_file_path.clone(),
                    "Unhandled node",
                    "",
                );
                continue;
            }
        };
        nodes.push(ir_node);
    }

    let state_variables = variables.pop_state();
    for (key, var) in state_variables {
        if !var.read && !key.starts_with("_") {
            compile_messages.create(
                MessageKind::Warning,
                var.location.clone(),
                relative_file_path.clone(),
                format!("Unused variable '{}'", key),
                format!(
                    "If this is intentional, prefix it with an underscore: '_{}'",
                    key
                ),
            );
        } else if !var.mutated && var.mutable {
            compile_messages.create(
                MessageKind::Warning,
                var.location.clone(),
                relative_file_path.clone(),
                format!("Variable does not need to be mutable '{}'", key),
                "",
            );
        }
    }

    return Ok(());
}

fn analyze_expression(
    compile_messages: &mut CompileMessages,
    relative_path: &Path,
    types: &FileTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    node: &Location,
    expression_info: Option<ExpressionInfo>,
) -> CompileResult<(IRExpressionInfo, Type)> {
    let expression_info = match expression_info {
        Some(expr) => expr,
        None => {
            let rt = match return_type {
                Some(t) => t,
                None => return Ok((IRExpressionInfo::void(), Type::void())),
            };
            if !rt.is_void() {
                compile_messages.create(
                    MessageKind::Error,
                    node.clone(),
                    relative_path.clone(),
                    format!("Expected type '{}' but no expression was provided.", rt),
                    "",
                );
            }
            return Ok((IRExpressionInfo::void(), Type::void()));
        }
    };

    use super::super::parser::Value;

    let (expression, data_type): (IRExpression, Type) = match &expression_info.expression {
        Expression::Call(path, arguments) => {
            let function = match types.get_function(relative_path, path)? {
                Some(f) => f,
                None => {
                    compile_messages.create(
                        MessageKind::Error,
                        node.clone(),
                        relative_path.clone(),
                        format!("Could not find path {}", path.components().join("::")),
                        "",
                    );
                    return Ok((IRExpressionInfo::void(), Type::void()));
                }
            };
            (
                IRExpression::Call(function.name.clone(), Vec::new()),
                function.return_type.clone(),
            )
        }
        Expression::GetVariable(path) => {
            let name = if &path.len() == &1 {
                path.first().unwrap()
            } else {
                panic!()
            };
            let variable = match variables.get(name, false) {
                Some(var) => var,
                None => {
                    compile_messages.create(
                        MessageKind::Error,
                        expression_info.location.clone(),
                        relative_path.clone(),
                        format!("Could not find variable named: '{}'", name),
                        "",
                    );
                    return Ok((IRExpressionInfo::void(), Type::void()));
                }
            };
            (
                IRExpression::GetVariable(variable.name.clone()),
                variable.data_type.clone(),
            )
        }
        Expression::Value(value) => match value {
            Value::Boolean(bool) => (IRExpression::Boolean(bool.clone()), value.default_type()),
            Value::StaticString(string) => {
                (IRExpression::Integer(string.clone()), value.default_type())
            }
            Value::Float(float) => {
                let rt: Type = match &return_type {
                    Some(rt) => {
                        if rt.is_float() {
                            rt.clone()
                        } else {
                            compile_messages.create(
                                MessageKind::Error,
                                node.clone(),
                                relative_path.clone(),
                                format!("Mismatched types, expected '{}', found float", rt),
                                "",
                            );
                            rt.clone()
                        }
                    }
                    None => value.default_type(),
                };

                (IRExpression::Float(float.clone()), rt)
            }
            Value::Integer(integer) => {
                let rt: Type = match &return_type {
                    Some(rt) => {
                        if rt.is_integer() {
                            rt.clone()
                        } else {
                            compile_messages.create(
                                MessageKind::Error,
                                node.clone(),
                                relative_path.clone(),
                                format!("Mismatched types, expected '{}', found integer", rt),
                                "",
                            );
                            rt.clone()
                        }
                    }
                    None => value.default_type(),
                };
                (IRExpression::Integer(integer.clone()), rt)
            }
        },
        _ => {
            compile_messages.create(
                MessageKind::Error,
                expression_info.location.clone(),
                relative_path.clone(),
                "Unhandled expression",
                "",
            );
            return Ok((IRExpressionInfo::void(), Type::void()));
        }
    };

    match return_type {
        Some(rt) => {
            if rt != &data_type {
                compile_messages.create(
                    MessageKind::Error,
                    expression_info.location.clone(),
                    relative_path.clone(),
                    format!("Wrong types, expected: '{}', got: '{}'", rt, &data_type),
                    "",
                );
            }
        }
        None => {}
    }

    return Ok((IRExpressionInfo::from(expression, &data_type), data_type));
}

pub fn convert_type(data_type: &Type) -> IRType {
    match data_type {
        Type::Base(base) => match base {
            BaseType::Int8 | BaseType::UInt8 => IRType::Integer(8),
            BaseType::Int16 | BaseType::UInt16 => IRType::Integer(16),
            BaseType::Int32 | BaseType::UInt32 => IRType::Integer(32),
            BaseType::Int64 | BaseType::UInt64 => IRType::Integer(64),
            BaseType::Boolean => IRType::Integer(1),
            BaseType::Void | BaseType::Never => IRType::Void,
            BaseType::Float32 => IRType::Float,
            BaseType::Float64 => IRType::Double,
            BaseType::StaticString => todo!(),
        },
        _ => todo!("{}", data_type),
    }
}
