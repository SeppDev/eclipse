use crate::compiler::{
    counter::NameCounter,
    errors::{CompileMessages, CompileResult, Location, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{
    node::{IRExpression, IRExpressionInfo, IRFunction, IRNode},
    parse_types,
    variables::Variables,
    IRProgram,
};

pub fn analyze(
    program: ParsedProgram,
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
) -> CompileResult<IRProgram> {
    let mut functions = Vec::new();
    let types = parse_types(compile_messages, name_counter, &program)?;
    println!("{:#?}", types);

    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    // analyze_file(compile_messages, &mut functions, parsed.main);

    return Ok(IRProgram { functions });
}

fn analyze_file(
    compile_messages: &mut CompileMessages,
    functions: &mut Vec<IRFunction>,
    mut file: ParsedFile,
) {
    loop {
        let (_, file) = match file.imports.pop() {
            Some((name, file)) => (name, file),
            None => break,
        };
        analyze_file(compile_messages, functions, file);
    }

    for info in file.functions {
        let (_, key, parameters, return_type, body) = match info.node {
            Node::Function {
                export,
                name,
                parameters,
                return_type,
                body,
            } => (export, name, parameters, return_type, body),
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
                        file.relative_path.clone(),
                        format!("Duplicate parameter '{}'", key),
                        "",
                    );
                }
            }
        }

        let mut nodes = Vec::new();
        analyze_body(
            compile_messages,
            &file.relative_path,
            &mut variables,
            &Some(return_type.clone()),
            body,
            &mut nodes,
        );

        if !return_type.is_void() {
            nodes.last().is_none().then(|| {
                compile_messages.create(
                    MessageKind::Error,
                    info.location.clone(),
                    file.relative_path.clone(),
                    format!("Expected return"),
                    "",
                );
            });
        }

        functions.push(IRFunction {
            name: key,
            parameters: parameters,
            return_type,
            body: nodes,
        })
    }
}

fn analyze_body(
    compile_messages: &mut CompileMessages,
    relative_path: &Path,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<NodeInfo>,
    nodes: &mut Vec<IRNode>,
) {
    use super::super::parser::Node;
    variables.create_state();

    let mut body = body.into_iter();

    loop {
        let info = match body.next() {
            Some(info) => info,
            None => break,
        };

        let ir_node: IRNode = match info.node {
            Node::Scope(body) => {
                analyze_body(
                    compile_messages,
                    relative_path,
                    variables,
                    return_type,
                    body,
                    nodes,
                );
                continue;
            }
            Node::Return(expression) => {
                let expression = analyze_expression(
                    compile_messages,
                    relative_path,
                    variables,
                    return_type,
                    &info.location,
                    expression,
                );
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
                    // errors.create(
                    //     MessageKind::Error,
                    //     relative_path.clone(),
                    //     info.location.clone(),
                    //     "Expected expression",
                    //     "",
                    // );
                    continue;
                }
                let expression = analyze_expression(
                    compile_messages,
                    relative_path,
                    variables,
                    &data_type,
                    &info.location,
                    expression,
                );
                let (current, result) = variables.insert(
                    &name,
                    mutable.clone(),
                    expression.data_type.clone(),
                    info.location.clone(),
                );

                match result {
                    Ok(_) => {}
                    Err(old) => {
                        let message = compile_messages.create(
                            MessageKind::Error,
                            old.location.clone(),
                            relative_path.clone(),
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
                        relative_path.clone(),
                        format!("Cannot asign to immutable variable '{}'", name.clone()),
                        "",
                    );
                }
                let expression = analyze_expression(
                    compile_messages,
                    relative_path,
                    variables,
                    &Some(variable.data_type),
                    &info.location,
                    expression,
                );

                IRNode::SetVariable(variable.name.clone(), expression)
            }
            _ => {
                compile_messages.create(
                    MessageKind::Error,
                    info.location.clone(),
                    relative_path.clone(),
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
        if !var.read {
            compile_messages.create(
                MessageKind::Warning,
                var.location.clone(),
                relative_path.clone(),
                format!("Unused variable '{}'", key),
                "",
            );
        } else if !var.mutated && var.mutable {
            compile_messages.create(
                MessageKind::Warning,
                var.location.clone(),
                relative_path.clone(),
                format!("Variable does not need to be mutable '{}'", key),
                "",
            );
        }
    }
}

fn analyze_expression(
    compile_messages: &mut CompileMessages,
    relative_path: &Path,
    variables: &mut Variables,
    return_type: &Option<Type>,
    node: &Location,
    expression: Option<ExpressionInfo>,
) -> IRExpressionInfo {
    let expression = match expression {
        Some(expr) => expr,
        None => {
            let rt = match return_type {
                Some(t) => t,
                None => return IRExpressionInfo::void(),
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
            return IRExpressionInfo::void();
        }
    };

    use super::super::parser::Value;

    let (ir_expression, data_type) = match &expression.expression {
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
                        expression.location.clone(),
                        relative_path.clone(),
                        format!("Could not find variable named: '{}'", name),
                        "",
                    );
                    return IRExpressionInfo::void();
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
                expression.location.clone(),
                relative_path.clone(),
                "Unhandled expression",
                "",
            );
            return IRExpressionInfo::void();
        }
    };

    match return_type {
        Some(rt) => {
            if rt != &data_type {
                compile_messages.create(
                    MessageKind::Error,
                    expression.location.clone(),
                    relative_path.clone(),
                    format!("Wrong types, expected: '{}', got: '{}'", rt, data_type),
                    "",
                );
            }
        }
        None => {}
    }

    return IRExpressionInfo::from(ir_expression, data_type);
}
