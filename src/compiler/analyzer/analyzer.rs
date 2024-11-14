use crate::compiler::{
    errors::{CompileMessages, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{
    node::{IRExpression, IRExpressionInfo, IRFunction, IRNode},
    variables::Variables,
    IRProgram,
};

pub fn analyze(parsed: &mut ParsedProgram, errors: &mut CompileMessages) -> IRProgram {
    let program = IRProgram::new();
    let mut functions = Vec::new();

    let std_path = Path::from("std");
    analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    let main_path = Path::from("src").join("main");
    analyze_file(parsed, &mut functions, errors, &parsed.main, &main_path);

    return program;
}

fn analyze_file(
    program: &ParsedProgram,
    functions: &mut Vec<IRFunction>,
    messages: &mut CompileMessages,
    file: &ParsedFile,
    path: &Path,
) {
    for (name, file) in &file.imported {
        analyze_file(program, functions, messages, file,  &path.join(name));
    }

    let mut file_messages = messages.create();
    file_messages.set_path(path.clone());

    for (name, info) in &file.functions {
        let (public, name, parameters, return_type, body) = match &info.node {
            Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => (public, name, parameters, return_type, body),
            _ => {
                // errors.create(
                //     MessageKind::Error,
                //     file.relative_path.clone(),
                //     format!("Expected function, got: {:#?}", info),
                //     "",
                //     info.location.clone(),
                // );
                continue;
            }
        };

        let mut variables = Variables::new(parameters.clone());
        let body = analyze_body(program, file, &mut variables, path, return_type, body);

        if !return_type.is_void() {
            match body.last() {
                Some(last) => {}
                None => {
                    file_messages.create(
                        MessageKind::Error,
                        info.location.clone(),
                        format!("Expected return"),
                        "",
                    );
                    continue;
                }
            }
        }

        functions.push(IRFunction {
            name: name.clone(),
            parameters: parameters.clone(),
            return_type: return_type.clone(),
            body,
        })
    }

    messages.push(file_messages);
}

fn analyze_body(
    program: &ParsedProgram,
    file: &ParsedFile,
    variables: &mut Variables,
    relative_path: &Path,
    return_type: &Type,
    nodes: &Vec<NodeInfo>,
) -> Vec<IRNode> {
    use super::super::parser::Node;
    let mut ir_nodes = Vec::new();

    for info in nodes {
        let ir_node: IRNode = match &info.node {
            Node::Return(expression) => {
                let expression = analyze_expression(
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
                let expression =
                    analyze_expression(program, file, variables, data_type, info, expression);
                IRNode::DeclareVariable(name.clone(), expression)
            }
            // Node::Call(path, arguments) => {
            // let file = parsed.get_file(path);
            // panic!("{:#?}", file);
            // }
            // Node::SetVariable { name, expression } => {
            //     // analyze_expression(parsed, file, namespace, return_type, expression)
            // }
            _ => panic!(), //program.throw_error("Unhandled node", &info.location),
        };
        ir_nodes.push(ir_node);
    }

    return ir_nodes;
}

fn analyze_expression(
    parsed: &ParsedProgram,
    file: &ParsedFile,
    variables: &Variables,
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
                panic!()
                // file.throw_error(
                //     format!("Expected type '{}' but no expression was provided.", rt),
                //     &node.location,
                // )
            }
            return IRExpressionInfo::void();
        }
    };

    use super::super::parser::Value;
    // use super::super::types::{BaseType, Type};

    let (ir_expression, data_type) = match &expression.expression {
        Expression::GetVariable(name) => {
            // let name = if &name.len() == &1 {
            //     name.components().first().unwrap()
            // } else {
            //     file.throw_error("Unhandled path", &expression.location)
            // };

            panic!("Getting variable named: {}", name)
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
                            // file.throw_error(
                            //     format!("Mismatched types, expected '{}', found float", rt),
                            //     &expression.location,
                            // )
                            panic!()
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
                            // file.throw_error(
                            //     format!("Mismatched types, expected '{}', found integer", rt),
                            //     &expression.location,
                            // )
                            panic!()
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
                // panic!()
                // file.throw_error(
                //     format!("Wrong types, expected: '{}', got: '{}'", rt, data_type),
                //     &expression.location,
                // );
            }
        }
        None => {}
    }
    // file.throw_error(format!("Expected integer type, got: '{}'", t), &expression.location);

    return IRExpressionInfo::from(ir_expression, data_type);
}
