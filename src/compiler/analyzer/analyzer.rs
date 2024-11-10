use crate::compiler::{
    parser::{Expression, ExpressionInfo, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{node::IRExpression, variables::Variables};

pub fn analyze(program: ParsedProgram) {
    let std = &program.standard;
    analyze_file(&program, std, &Path::from("std"));

    let main = &program.main;
    analyze_file(&program, main, &Path::from("src").join("main"));
}

fn analyze_file(program: &ParsedProgram, file: &ParsedFile, path: &Path) {
    for (name, info) in &file.functions {
        let (public, parameters, return_type, body) = match &info.node {
            crate::compiler::parser::Node::Function {
                public,
                parameters,
                return_type,
                body,
            } => (public, parameters, return_type, body),
            _ => panic!(),
        };

        let mut variables = Variables::new(parameters.clone());
        analyze_body(program, file, &mut variables, path, return_type, body);
    }

    for (name, imported) in &file.imported {
        analyze_file(program, imported, &path.join(name));
    }
}

fn analyze_body(
    program: &ParsedProgram,
    file: &ParsedFile,
    variables: &mut Variables,
    namespace: &Path,
    return_type: &Type,
    nodes: &Vec<NodeInfo>,
) {
    use super::super::parser::Node;

    for info in nodes {
        let _ = match &info.node {
            Node::Return(expression) => {
                analyze_expression(
                    program,
                    file,
                    variables,
                    namespace,
                    &Some(return_type.clone()),
                    info,
                    expression,
                );
            }
            Node::DefineVariable {
                name,
                mutable,
                data_type,
                expression,
            } => {
                analyze_expression(
                    program, file, variables, namespace, data_type, info, expression,
                );
            }
            // Node::SetVariable { name, expression } => {
            //     // analyze_expression(program, file, namespace, return_type, expression)
            // }
            _ => file.throw_error("Unhandled node", &info.location),
        };
    }
}

fn analyze_expression(
    program: &ParsedProgram,
    file: &ParsedFile,
    variables: &Variables,
    namespace: &Path,
    return_type: &Option<Type>,
    node: &NodeInfo,
    expression: &Option<ExpressionInfo>,
) -> Option<IRExpression> {
    let expression = match expression {
        Some(expr) => expr,
        None => {
            let rt = match return_type {
                Some(t) => t,
                None => return None,
            };
            if !rt.is_void() {
                file.throw_error(
                    format!("Expected type '{}' but did not receive any expression", rt),
                    &node.location,
                )
            }
            return None;
        }
    };

    // use super::super::parser::Value;
    // use super::super::types::{BaseType, Type};

    let ir_expression = match &expression.expression {
        Expression::Value(value) => IRExpression::Value(value.clone()),
        _ => file.throw_error("Unhandled expression", &expression.location),
    };

    return Some(ir_expression);
}
