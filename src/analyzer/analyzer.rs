use std::collections::HashMap;

use crate::{
    analyzer::parse_datastructures,
    parser::{ASTNode, Expression, Modules, Node, Path, Type, Value},
    AnalyzeResult, CompileError,
};

use super::{
    program::{Function, IRExpression, IRNode, Variables},
    Types,
};

pub fn analyze(modules: Modules) -> AnalyzeResult<()> {
    // println!("{:#?}", modules);
    let types = parse_datastructures(&modules)?;
    // println!("{:#?}", types);
    let functions = parse(&types, modules)?;

    let mut main_path = Path::new(String::from("src"));
    main_path.add(String::from("main"));
    main_path.add(String::from("main"));
    let main = match functions.get(&main_path) {
        Some(f) => f,
        None => {
            return Err(CompileError::new(
                format!("Failed to provide a main function"),
                0,
            ))
        }
    };
    match &main.return_type {
        Some(t) => {
            if !t.is_integer() {
                return Err(CompileError::new(
                    format!("return type of main must be a integer type"),
                    0,
                ));
            }
        }
        None => {}
    };
    println!("{:#?}", functions);

    todo!()
}

fn parse(types: &Types, modules: Modules) -> AnalyzeResult<HashMap<Path, Function>> {
    let mut functions = HashMap::new();
    for (relative_path, nodes) in modules {
        for ast in nodes {
            match ast.node {
                Node::Function {
                    export,
                    is_unsafe,
                    name,
                    generics,
                    parameters,
                    return_type,
                    body,
                } => {
                    let mut variables = Variables::new();
                    let nodes = parse_body(types, &return_type, &mut variables, body)?;
                    let mut path = relative_path.clone();
                    path.add(name);

                    let function = Function {
                        parameters,
                        return_type,
                        body: nodes,
                    };
                    functions.insert(path, function);
                }
                _ => continue,
            }
        }
    }

    return Ok(functions);
}

fn parse_body(
    types: &Types,
    return_type: &Option<Type>,
    variables: &mut Variables,
    nodes: Vec<ASTNode>,
) -> AnalyzeResult<Vec<IRNode>> {
    let mut tree = Vec::new();
    for ast in nodes {
        tree.push(parse_node(types, return_type, variables, ast)?);
    }

    return Ok(tree);
}

fn parse_node(
    types: &Types,
    return_type: &Option<Type>,
    variables: &mut Variables,
    ast: ASTNode,
) -> AnalyzeResult<IRNode> {
    match ast.node {
        Node::Scope { is_unsafe, body } => {
            let body = parse_body(types, return_type, variables, body)?;
            return Ok(IRNode::Scope {
                is_unsafe: false,
                body,
            });
        }
        Node::Return(expression) => {
            let expression = match expression {
                Some(expr) => expr,
                None => return Ok(IRNode::Return(None))
            };
            let ir = match expression {
                Expression::Value(value) => IRExpression::Value(value),
                _ => todo!(),
            };

            return Ok(IRNode::Return(Some(ir)));
        },
        node => todo!("{:#?}", node)
    }
}
