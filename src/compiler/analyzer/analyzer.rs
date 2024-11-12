use crate::compiler::{
    parser::{Expression, ExpressionInfo, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{
    node::{IRExpression, IRExpressionInfo, IRFunction, IRNode},
    variables::Variables,
    IRProgram,
};

fn push_functions(program: &mut IRProgram, functions: Vec<IRFunction>) {
    for function in functions {
        program.functions.push(function);
    }
}

pub fn analyze(parsed: ParsedProgram) -> IRProgram {
    let mut program = IRProgram::new();

    let std = &parsed.standard;
    push_functions(&mut program, analyze_file(&parsed, std, &Path::from("std")));

    let main = &parsed.main;
    push_functions(
        &mut program,
        analyze_file(&parsed, main, &Path::from("src").join("main")),
    );

    return program;
}

fn analyze_file(parsed: &ParsedProgram, file: &ParsedFile, path: &Path) -> Vec<IRFunction> {
    let mut functions = Vec::new();

    for (name, info) in &file.functions {
        let (public, name, parameters, return_type, body) = match &info.node {
            crate::compiler::parser::Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => (public, name, parameters, return_type, body),
            _ => file.throw_error(
                format!("Expected function, got: {:#?}", info),
                &info.location,
            ),
        };

        let mut variables = Variables::new(parameters.clone());
        let body = analyze_body(parsed, file, &mut variables, path, return_type, body);
        functions.push(IRFunction {
            name: name.clone(),
            parameters: parameters.clone(),
            return_type: return_type.clone(),
            body,
        })
    }

    for (name, imported) in &file.imported {
        for function in analyze_file(parsed, imported, &path.join(name)) {
            functions.push(function);
        }
    }

    return functions;
}

fn analyze_body(
    parsed: &ParsedProgram,
    file: &ParsedFile,
    variables: &mut Variables,
    namespace: &Path,
    return_type: &Type,
    nodes: &Vec<NodeInfo>,
) -> Vec<IRNode> {
    use super::super::parser::Node;
    let mut ir_nodes = Vec::new();

    for info in nodes {
        let ir_node: IRNode = match &info.node {
            Node::Return(expression) => {
                let expression = analyze_expression(
                    parsed,
                    file,
                    variables,
                    namespace,
                    &Some(return_type.clone()),
                    info,
                    expression,
                );
                IRNode::Return(expression)
            }
            Node::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => {
                if expression.is_none() {
                    file.throw_error("Expected expression", &info.location)
                }
                let expression = analyze_expression(
                    parsed, file, variables, namespace, data_type, info, expression,
                );
                IRNode::DeclareVariable(name.clone(), expression)
            }
            Node::Call(path, arguments) => {
                let file = parsed.get_file(path);
                panic!("{:#?}", file);
            }
            // Node::SetVariable { name, expression } => {
            //     // analyze_expression(parsed, file, namespace, return_type, expression)
            // }
            _ => file.throw_error("Unhandled node", &info.location),
        };
        ir_nodes.push(ir_node);
    }

    return ir_nodes;
}

fn analyze_expression(
    parsed: &ParsedProgram,
    file: &ParsedFile,
    variables: &Variables,
    namespace: &Path,
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
                file.throw_error(
                    format!("Expected type '{}' but did not receive any expression", rt),
                    &node.location,
                )
            }
            return IRExpressionInfo::void();
        }
    };

    use super::super::parser::Value;
    // use super::super::types::{BaseType, Type};

    let (ir_expression, data_type) = match &expression.expression {
        Expression::GetVariable(name) => {
            let name = if &name.len() == &1 {
                name.components().first().unwrap()
            } else {
                file.throw_error("Unhandled path", &expression.location)
            };

            panic!("Getting variable named: {}", name)
        }
        Expression::Value(value) => {
            match value {
                Value::Boolean(bool) => (IRExpression::Boolean(bool.clone()), value.default_type()),
                Value::StaticString(string) => (IRExpression::Integer(string.clone()), value.default_type()),
                Value::Float(float) => {
                    let rt: Type = match &return_type {
                        Some(rt) => {
                            if rt.is_float() {
                                rt.clone()
                            } else {
                                file.throw_error(format!("Mismatched types, expected '{}', found float", rt), &expression.location)
                            }

                        },
                        None => value.default_type()
                    };

                    (IRExpression::Float(float.clone()), rt)
                },
                Value::Integer(integer) => {
                    let rt: Type = match &return_type {
                        Some(rt) => {
                            if rt.is_integer() {
                                rt.clone()
                            } else {
                                file.throw_error(format!("Mismatched types, expected '{}', found integer", rt), &expression.location)
                            }

                        },
                        None => value.default_type()
                    };

                    (IRExpression::Integer(integer.clone()), rt)
                },
            }
        }
        _ => file.throw_error("Unhandled expression", &expression.location),
    };

    match return_type {
        Some(rt) => {
            if rt != &data_type {
                file.throw_error(
                    format!("Wrong types, expected: '{}', got: '{}'", rt, data_type),
                    &expression.location,
                );
            }
        }
        None => {}
    }
    // file.throw_error(format!("Expected integer type, got: '{}'", t), &expression.location);

    return IRExpressionInfo::from(ir_expression, data_type);
}
