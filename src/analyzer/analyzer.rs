use std::collections::HashMap;

use crate::{
    analyzer::nodes::Function, ASTModule, ASTNode, AnalyzeResult, Expression, Node, Path, Type,
    Value,
};

use super::{
    nodes::{IRExpression, IRModule, IRNode}, types::{get_function_types, FunctionTypes}, variables::Variables
};

pub fn analyze(module: ASTModule) -> AnalyzeResult<(IRModule, FunctionTypes)> {
    let types = get_function_types(&module)?;

    let main_path = Path::from(String::from("main"));
    let main = handle_module(module, &types, main_path)?;

    return Ok((main, types));
}

fn handle_module(
    module: ASTModule,
    types: &FunctionTypes,
    current_path: Path,
) -> AnalyzeResult<IRModule> {
    let mut submodules: HashMap<String, (bool, IRModule)> = HashMap::new();
    let mut functions = HashMap::new();

    for node in module.body {
        match node.node {
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                let mut variables = Variables::new(parameters.clone());
                let (nodes, returned) =
                    handle_scope(types, &mut variables, &Some(return_type.clone()), body)?;

                assert!(returned == return_type);

                let function = Function {
                    parameters,
                    return_type,
                    nodes,
                };
                functions.insert(name, function);
            }
            _ => continue,
        }
    }

    for (name, (export, ast_module)) in module.submodules {
        let mut sub_path = current_path.clone();
        sub_path.add(name.clone());

        let ir_module = handle_module(ast_module, types, sub_path)?;
        submodules.insert(name, (export, ir_module));
    }

    return Ok(IRModule {
        submodules,
        body: functions,
    });
}

fn handle_scope(
    types: &FunctionTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<(Vec<IRNode>, Type)> {
    let mut nodes = Vec::new();
    variables.create_state();

    let mut returned = Type::Base(crate::BaseType::Void);

    for node in body {
        let ir = match node.node {
            Node::DefineVariable {
                mutable,
                name,
                data_type,
                expression,
            } => {
                let expression =
                    handle_expression(types, variables, &data_type, expression.unwrap())?;

                variables.insert(
                    name.clone(),
                    mutable,
                    Some(expression.parse_type()),
                )?;
                IRNode::DefineVariable { name, expression }
            }
            Node::Return(expression) => {
                let ir = match expression {
                    Some(expr) => {
                        let expression = handle_expression(types, variables, return_type, expr)?;
                        returned = expression.parse_type();
                        IRNode::Return(Some(expression))
                    }
                    None => IRNode::Return(None),
                };
                nodes.push(ir);
                break;
            }
            _ => continue,
        };
        nodes.push(ir);
    }

    variables.pop_state();
    return Ok((nodes, returned));
}

fn handle_expression(
    types: &FunctionTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    expression: Expression,
) -> AnalyzeResult<IRExpression> {
    return match expression {
        Expression::GetVariable(name) => {
            let var = variables.get(&name)?;
            if &var.data_type != return_type {
                panic!(
                    "WRONG RETURN TYPES: {:?} != {:?}",
                    &var.data_type, return_type
                );
            }
            Ok(IRExpression::GetVariable(
                var.data_type.clone().unwrap(),
                name,
            ))
        }
        Expression::Value(value) => match value {
            Value::Integer(signed, integer) => match return_type {
                Some(t) => {
                    let c = t.clone();
                    assert!(c.is_integer());
                    Ok(IRExpression::Value(c, value))
                }
                None => Ok(IRExpression::Value(
                    Type::Base(crate::BaseType::Int32),
                    value,
                )),
            },
            _ => todo!(),
        },
        _ => todo!(),
    };
}
