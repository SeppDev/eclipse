use crate::compiler::{
    errors::{CompileMessages, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo, ParsedFile},
    program::ParsedProgram,
    types::{BaseType, Type},
};

use super::{
    node::{IRExpression, IRExpressionInfo, IRFunction, IRNode},
    variables::Variables,
    IRProgram,
};

pub fn analyze(parsed: &mut ParsedProgram, compile_messages: &mut CompileMessages) -> IRProgram {
    let mut functions = Vec::new();

    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    analyze_file(compile_messages, &mut functions, parsed, &parsed.main);

    return IRProgram { functions };
}

fn analyze_file(
    compile_messages: &mut CompileMessages,
    functions: &mut Vec<IRFunction>,
    program: &ParsedProgram,
    file: &ParsedFile,
) {
    // for (name, file) in &file.imports {
    //     analyze_file(program, functions, messages, file, &path.join(name));
    // }

    for (name, info) in &file.functions {
        let (public, name, parameters, return_type, body) = match &info.node {
            Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => (public, name, parameters, return_type, body),
            _ => continue,
        };

        let mut variables = Variables::new(parameters.clone());
        let body = analyze_body(
            compile_messages,
            program,
            file,
            &mut variables,
            return_type,
            body,
        );

        if !return_type.is_void() {
            body.last().is_none().then(|| {
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
            name: name.clone(),
            parameters: parameters.clone(),
            return_type: return_type.clone(),
            body,
        })
    }
}

fn analyze_body(
    compile_messages: &mut CompileMessages,
    program: &ParsedProgram,
    file: &ParsedFile,
    variables: &mut Variables,
    return_type: &Type,
    nodes: &Vec<NodeInfo>,
) -> Vec<IRNode> {
    use super::super::parser::Node;
    let mut ir_nodes = Vec::new();
    variables.create_state();

    for info in nodes {
        let ir_node: IRNode = match &info.node {
            Node::Return(expression) => {
                let expression = analyze_expression(
                    compile_messages,
                    program,
                    file,
                    variables,
                    &Some(return_type.clone()),
                    info,
                    expression,
                );
                ir_nodes.push(IRNode::Return(expression));
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
                    program,
                    file,
                    variables,
                    data_type,
                    info,
                    expression,
                );
                variables
                    .insert(name.clone(), mutable.clone(), expression.data_type.clone())
                    .unwrap_or_else(|_| {
                        compile_messages.create(
                            MessageKind::Error,
                            info.location.clone(),
                            file.relative_path.clone(),
                            format!("'{}' is already declared", name.clone()),
                            "",
                        );
                    });
                IRNode::DeclareVariable(name.clone(), expression)
            }
            Node::SetVariable { name, expression } => {
                let variable = match variables.get(name) {
                    Some(var) => var.clone(),
                    None => todo!(),
                };
                if !variable.mutable {
                    compile_messages.create(
                        MessageKind::Error,
                        info.location.clone(),
                        file.relative_path.clone(),
                        format!("Cannot asign to immutable variable '{}'", name.clone()),
                        "",
                    );
                    continue;
                }
                let expression = analyze_expression(
                    compile_messages,
                    program,
                    file,
                    variables,
                    &Some(variable.data_type),
                    info,
                    expression,
                );
                IRNode::SetVariable(name.clone(), expression)
            }
            _ => {
                compile_messages.create(
                    MessageKind::Error,
                    info.location.clone(),
                    file.relative_path.clone(),
                    "Unhandled node",
                    "",
                );
                continue;
            }
        };
        ir_nodes.push(ir_node);
    }

    variables.pop_state();

    return ir_nodes;
}

fn analyze_expression(
    compile_messages: &mut CompileMessages,
    program: &ParsedProgram,
    file: &ParsedFile,
    variables: &mut Variables,
    return_type: &Option<Type>,
    node: &NodeInfo,
    expression: &Option<ExpressionInfo>,
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
                    node.location.clone(),
                    file.relative_path.clone(),
                    format!("Expected type '{}' but no expression was provided.", rt),
                    "",
                );
            }
            return IRExpressionInfo::void();
        }
    };

    use super::super::parser::Value;
    // use super::super::types::{BaseType, Type};

    let (ir_expression, data_type) = match &expression.expression {
        Expression::GetVariable(path) => {
            let name = if &path.len() == &1 {
                path.first().unwrap()
            } else {
                panic!()
            };
            let variable = match variables.get(name) {
                Some(var) => var,
                None => {
                    compile_messages.create(
                        MessageKind::Error,
                        node.location.clone(),
                        file.relative_path.clone(),
                        format!("Could not find variable named: '{}'", name),
                        "",
                    );
                    return IRExpressionInfo::void();
                }
            };
            (
                IRExpression::GetVariable(name.clone()),
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
                                node.location.clone(),
                                file.relative_path.clone(),
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
                                node.location.clone(),
                                file.relative_path.clone(),
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
        _ => panic!(), //file.throw_error("Unhandled expression", &expression.location),
    };

    match return_type {
        Some(rt) => {
            if rt != &data_type {
                compile_messages.create(
                    MessageKind::Error,
                    expression.location.clone(),
                    file.relative_path.clone(),
                    format!("Wrong types, expected: '{}', got: '{}'", rt, data_type),
                    "",
                );
            }
        }
        None => {}
    }

    return IRExpressionInfo::from(ir_expression, data_type);
}
