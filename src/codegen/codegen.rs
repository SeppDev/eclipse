use std::path::PathBuf;

use crate::{
    parser::{ASTNode, Expression, Node, Program, Value},
    CompileError, FILE_EXTENSION,
};

use super::{function::FunctionInfo, node::IRProgram, IRFunction, IRNode};

fn create_function_path(relative_path: &PathBuf, name: &String) -> String {
    let mut function_path = relative_path
        .to_str()
        .unwrap()
        .replace(format!(".{}", FILE_EXTENSION).as_str(), "")
        .replace("/", ".");

    function_path.push_str(format!(".{}", name).as_str());

    return function_path;
}

pub fn generate(program: Program) -> Result<IRProgram, CompileError> {
    let mut ir_program = IRProgram::new();

    for (path, module) in program.modules.iter() {
        for node in &module.body {
            match &node.node {
                #[allow(unused)]
                Node::Function {
                    export,
                    is_unsafe,
                    name,
                    parameters,
                    return_type,
                    body,
                } => {
                    ir_program.functions.insert(
                        create_function_path(&path, &name),
                        (parameters.clone(), return_type.clone()),
                    );
                }
                Node::Import(_, _) => continue,
                node => panic!("Expected function got: {:?}", node),
            }
        }
    }

    for (path, module) in program.modules {
        match parse_root(module.body, path, &mut ir_program) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
    }

    return Ok(ir_program);
}

pub fn parse_root(
    nodes: Vec<ASTNode>,
    relative_path: PathBuf,
    ir_program: &mut IRProgram,
) -> Result<(), CompileError> {
    let function_path = relative_path
        .to_str()
        .unwrap()
        .replace(format!(".{}", FILE_EXTENSION).as_str(), "")
        .replace("/", ".");

    for ast_node in nodes {
        let node = ast_node.node;
        match node {
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => {
                let mut function = FunctionInfo::new();
                match parse_body(body, is_unsafe.to_owned(), &mut function) {
                    Ok(nodes) => {
                        let result = ir_program.body.insert(
                            format!("{}.{}", function_path, name),
                            IRFunction {
                                stack_size: 16,
                                body: nodes,
                            },
                        );
    
                        match result {
                            Some(_) => panic!(),
                            None => continue,
                        }
                    }
                    Err(error) => panic!("{:?}", error),
                }
            },
            Node::Import(_, _) => continue,
            _ => panic!("Expected function"),
        }
    }

    return Ok(());
}

pub fn parse_body(body: Vec<ASTNode>, is_unsafe: bool, function: &mut FunctionInfo) -> Result<Vec<IRNode>, CompileError> {
    let mut tree = Vec::new();

    for node in body {
        match node.node {
            Node::Scope { is_unsafe, body } => match parse_body(body, is_unsafe, function) {
                Ok(body) => {}
                Err(error) => return Err(error),
            },
            Node::DefineVariable {
                mutable,
                name,
                var_type,
                expression,
            } => {
                match parse_expression(expression.unwrap(), &is_unsafe, function) {
                    Ok(node) => tree.push(node),
                    Err(error) => return Err(error)
                }
                // tree.push(IRNode::StoreValue {
                //     offset: 8,
                //     value: 4321,
                // };
            },
            // node => todo!("{:#?}", node),
            _ => continue,
        }
    }

    return Ok(tree);
}

pub fn parse_expression(expression: Expression, is_unsafe: &bool, function: &mut FunctionInfo) -> Result<IRNode, CompileError> {
    // match expression {
    //     Expression::Value(value) => match value {
    //         Value::Integer(integer) => 
    //     }
    //     expr => todo!("{:#?}", expr)
    // }

    todo!()
}   