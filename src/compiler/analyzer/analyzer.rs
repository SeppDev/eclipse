use crate::compiler::{
    parser::{Expression, ExpressionInfo, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{
    node::{IRExpression, IRFunction, IRNode},
    variables::Variables,
    IRProgram,
};

fn push_functions(program: &mut IRProgram, functions: Vec<IRFunction>) {
    for function in functions {
        let key = program.generate();
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
        let body = analyze_body(parsed, file, &mut variables, path, return_type, body);
        functions.push(IRFunction {
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
    nodes: Vec<NodeInfo>,
) -> Vec<IRNode> {
    use super::super::parser::Node;
    let mut ir_nodes = Vec::new();

    for info in nodes {
        let ir_node: IRNode = match info.node {
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
            // Node::DefineVariable {
            //     name,
            //     mutable,
            //     data_type,
            //     expression,
            // } => {
            //     let expression = analyze_expression(
            //         parsed, file, variables, namespace, data_type, info, expression,
            //     );
            //     IRNode::DefineVariable(name.clone(), )
            // }
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
