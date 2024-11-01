use crate::{
    analyzer::nodes::IRFunction,
    types::{ASTNode, BaseType, Expression, Node, Path, Type, Value},
    ASTModule, AnalyzeResult,
};

use super::{
    get_function_types,
    nodes::{IRExpression, IRNode},
    variables::Variables,
    IRModule, IRProgram, ModuleTypes, RandomString,
};

pub fn analyze(module: ASTModule, random_string: &mut RandomString) -> AnalyzeResult<IRProgram> {
    let types = get_function_types(&module, random_string)?;

    let mut modules = Vec::new();
    handle_module(random_string, &mut modules, module, &types, Path::new())?;

    return Ok(IRProgram { modules, types });
}

fn handle_module(
    random: &mut RandomString,
    modules: &mut Vec<IRModule>,
    module: ASTModule,
    types: &ModuleTypes,
    current_path: Path,
) -> AnalyzeResult<()> {
    let mut functions = Vec::new();

    for node in module.body {
        #[allow(unused)]
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
                let fname = types
                    .get_function(&Path::new(), current_path.clone(), &name)?
                    .name
                    .clone();
                let mut nodes =
                    handle_scope(types, &current_path, body, &return_type, &mut variables)?;

                match nodes.last() {
                    Some(n) => {
                        if !n.is_return() {
                            assert!(return_type.is_void());
                            nodes.push(IRNode::Return(None))
                        }
                    }
                    None => {
                        assert!(return_type.is_void());
                        nodes.push(IRNode::Return(None))
                    }
                }

                let function = IRFunction {
                    name: fname,
                    parameters,
                    return_type,
                    nodes,
                };
                functions.push(function);
            }
            _ => continue,
        }
    }

    for (name, (_, ast_module)) in module.submodules {
        let mut sub_path = current_path.clone();
        sub_path.add(name);

        handle_module(random, modules, ast_module, types, sub_path)?;
    }

    modules.push(IRModule { functions });
    return Ok(());
}

fn handle_scope(
    types: &ModuleTypes,
    current_path: &Path,
    body: Vec<ASTNode>,
    return_type: &crate::types::Type,
    variables: &mut Variables,
) -> AnalyzeResult<Vec<IRNode>> {
    variables.create_state();
    let mut nodes = Vec::new();

    for node in body {
        let new_node: IRNode = match node.node {
            Node::Loop(nodes) => IRNode::Loop(handle_scope(
                types,
                current_path,
                nodes,
                return_type,
                variables,
            )?),
            Node::Return(expression) => match expression {
                Some(expr) => {
                    let (expression, data_type) = handle_expression(
                        types,
                        current_path,
                        Some(return_type.clone()),
                        variables,
                        expr,
                    )?;
                    assert!(&data_type == return_type);
                    IRNode::Return(Some(expression))
                }
                None => {
                    assert!(return_type.is_void());
                    IRNode::Return(None)
                }
            },
            Node::DefineVariable {
                mutable,
                name,
                data_type,
                expression,
            } => {
                let (expression, var_type) = handle_expression(
                    types,
                    current_path,
                    data_type.clone(),
                    variables,
                    expression.unwrap(),
                )?;
                let variable = variables.insert(name.clone(), mutable, var_type.clone())?;

                IRNode::DefineVariable(variable.name.clone(), var_type, expression)
            }
            Node::SetVariable(name, expression) => {
                let variable = variables.get(&name)?;
                assert!(variable.mutable == true);
                let var_type = variable.data_type.clone();

                let (irexpression, data_type) =
                    handle_expression(types, current_path, Some(var_type), variables, expression)?;

                // if variable.data_type.is_none() {
                // variables.change_type(&name, data_type.clone())?;
                // }
                IRNode::SetVariable(variable.name.clone(), data_type, irexpression)
            }
            Node::Expression(expression) => {
                let (expr, data_type) = handle_expression(
                    types,
                    current_path,
                    None,
                    variables,
                    expression,
                )?;

                match expr {
                    IRExpression::Call(path, arguments) => IRNode::Call(path, data_type, arguments),
                    _ => todo!(),
                }
            }
            Node::Break => IRNode::Break,
            t => panic!("{:#?}", t),
        };

        if new_node.is_return() {
            nodes.push(new_node);
            break;
        }
        nodes.push(new_node);
    }

    variables.pop_state();
    return Ok(nodes);
}

fn handle_expression(
    types: &ModuleTypes,
    current_path: &Path,
    return_type: Option<Type>,
    variables: &Variables,
    expression: Expression,
) -> AnalyzeResult<(IRExpression, Type)> {
    let (ir, data_type): (IRExpression, Type) = match expression {
        Expression::Value(value) => {
            let data_type = match return_type {
                Some(t) => match value {
                    Value::Integer(_, _) => {
                        assert!(t.is_integer());
                        t
                    }
                    Value::Float(_) => {
                        assert!(t.is_float());
                        t
                    }
                    Value::Boolean(_) => {
                        assert!(t.is_bool());
                        t
                    }
                    _ => todo!(),
                },
                None => match value {
                    Value::Integer(_, _) => Type::Base(BaseType::Int32),
                    Value::Float(_) => Type::Base(BaseType::Float64),
                    Value::Boolean(_) => Type::Base(BaseType::Boolean),
                    _ => todo!(),
                },
            };

            // match data_type.integer_info() {
            //     Some((signed, bits)) => {
            //         let mut max_value: usize = 2 ^ bits;
            //         match value {
            //             Value::Integer(minus, value) => if signed {
            //                 let value = i64::
            //                 let max_value = max_value as isize / 2 - 1;
            //                 assert!(value <= max_value);
            //                 assert!(value > -max_value);
            //             } else {
            //                 assert!(value <= max_value)
            //             },
            //             _ => panic!()
            //         }
            //     },
            //     None => {}
            // }

            (IRExpression::Value(value), data_type)
        }
        Expression::BinaryOperation(expr1, operator, expr2) => {
            let a = *expr1;
            let b = *expr2;
            let (first, t1) = handle_expression(types, current_path, return_type, variables, a)?;
            let (second, t2) = handle_expression(types, current_path, Some(t1), variables, b)?;

            (
                IRExpression::BinaryOperation(Box::new(first), operator, Box::new(second)),
                t2,
            )
        }
        Expression::Call(mut path, arguments) => {
            let name = path.components.pop().unwrap();
            let function = types.get_function(current_path, path, &name)?;
            let mut args = Vec::new();

            let mut used = 0;
            for (i, arg) in arguments.into_iter().enumerate() {
                let param = match function.parameters.get(i) {
                    Some(param) => param,
                    None => panic!("Too many arguments!"),
                };

                used += 1;
                let a =
                    handle_expression(types, current_path, Some(param.1.clone()), variables, arg)?;
                assert!(a.1 == param.1);
                args.push(a);
            }
            assert!(function.parameters.len() == used, "Too few arguments");

            match return_type {
                Some(t) => assert!(t == function.return_type),
                None => {}
            }

            (
                IRExpression::Call(function.name.clone(), args),
                function.return_type.clone(),
            )
        }
        Expression::GetVariable(name) => {
            let variable = variables.get(&name)?;
            let data_type = variable.clone().data_type;

            (IRExpression::GetVariable(variable.name.clone()), data_type)
        }
        t => todo!("{:?}", t),
    };

    return Ok((ir, data_type));
}
