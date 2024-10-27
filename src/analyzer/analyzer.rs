use std::collections::HashMap;

use crate::{
    analyzer::nodes::IRFunction, ASTModule, ASTNode, AnalyzeResult, Expression, Node, Path, Type,
    Value,
};

use super::{
    nodes::{IRExpression, IRNode},
    types::{get_function_types, FunctionTypes},
    variables::Variables,
    IRModule, IRProgram,
};

pub fn analyze(module: ASTModule) -> AnalyzeResult<IRProgram> {
    let types = get_function_types(&module)?;

    let main_path = Path::from(String::from("main"));
    let mut modules = HashMap::new();
    handle_module(&mut modules, module, &types, main_path)?;

    return Ok(IRProgram { modules, types });
}

fn handle_module(
    modules: &mut HashMap<Path, IRModule>,
    module: ASTModule,
    types: &FunctionTypes,
    current_path: Path,
) -> AnalyzeResult<()> {
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
                    handle_scope(&current_path, types, &mut variables, &Some(return_type.clone()), body)?;

                assert!(returned == return_type);

                let function = IRFunction {
                    parameters,
                    return_type,
                    nodes,
                };
                functions.insert(name, function);
            }
            _ => continue,
        }
    }

    for (name, (_, ast_module)) in module.submodules {
        let mut sub_path = current_path.clone();
        sub_path.add(name.clone());

        handle_module(modules, ast_module, types, sub_path)?;
    }

    modules.insert(current_path, IRModule { functions });
    return Ok(());
}

fn handle_scope(
    current_path: &Path,
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
                let (expression, expr_type) =
                    handle_expression(types, variables, &data_type, expression.unwrap(), current_path)?;

                variables.insert(name.clone(), mutable, data_type.clone())?;
                IRNode::DefineVariable {
                    name,
                    expression,
                    data_type: expr_type,
                }
            }
            Node::Return(expression) => {
                let ir = match expression {
                    Some(expr) => {
                        let (expression, expr_type) = handle_expression(types, variables, return_type, expr, current_path)?;
                        returned = expr_type;
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
    current_path: &Path
) -> AnalyzeResult<(IRExpression, Type)> {
    return match expression {
        Expression::GetVariable(name) => {
            let var = variables.get(&name)?;
            if &var.data_type != return_type {
                panic!(
                    "WRONG RETURN TYPES: {:?} != {:?}",
                    &var.data_type, return_type
                );
            }
            Ok((
                IRExpression::GetVariable(name),
                var.data_type.clone().unwrap(),
            ))
        }
        Expression::Value(value) => match value {
            Value::Integer(signed, integer) => match return_type {
                Some(t) => Ok((IRExpression::Value(value), t.clone())),
                None => Ok((IRExpression::Value(value), Type::Base(crate::BaseType::Int32))),
            },
            Value::Float(_) => match return_type {
                Some(t) => Ok((IRExpression::Value(value), t.clone())),
                None => Ok((IRExpression::Value(value), Type::Base(crate::BaseType::Float64))),
            }
            _ => todo!(),
        },
        Expression::Call(relative_path, arguments) => {
            let (full_path, function) = types.get_function(current_path, relative_path)?;
            match return_type {
                Some(t) => assert!(t == &function.return_type),
                None => {}
            }
            Ok((IRExpression::Call(full_path, Vec::new()), function.return_type.clone()))
        }
        _ => todo!(),
    };
}
