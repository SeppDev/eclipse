use crate::{
    analyzer::nodes::IRFunction, ASTModule, ASTNode, AnalyzeResult, BaseType, Expression, Node,
    Path, Type, Value,
};

use super::{
    get_function_types,
    nodes::{IRExpression, IRNode},
    variables::{RandomString, Variables},
    IRModule, IRProgram, ModuleTypes,
};

pub fn analyze(module: ASTModule) -> AnalyzeResult<IRProgram> {
    let mut random_string = RandomString::new();
    let types = get_function_types(&module, &mut random_string)?;

    let mut modules = Vec::new();
    handle_module(
        &mut random_string,
        &mut modules,
        module,
        &types,
        Path::new(),
    )?;

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
                let nodes = handle_scope(types, &current_path, body, &return_type, &mut variables)?;

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
    return_type: &Type,
    variables: &mut Variables,
) -> AnalyzeResult<Vec<IRNode>> {
    variables.create_state();
    let mut nodes = Vec::new();

    for node in body {
        let new_node: IRNode = match node.node {
            Node::Return(expression) => match expression {
                Some(expr) => {
                    let (expression, _) = handle_expression(
                        types,
                        current_path,
                        Some(return_type.clone()),
                        variables,
                        expr,
                    )?;
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
                let variable = variables.insert(name.clone(), mutable, data_type)?;

                IRNode::DefineVariable {
                    name: variable.name.clone(),
                    data_type: var_type,
                    expression: expression,
                }
            }
            Node::Expression(expression) => {
                let (expression, data_type) = handle_expression(types, current_path, None, variables, expression)?;
                IRNode::Expression(expression, data_type)
            }
            t => panic!("{:#?}", t),
        };
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
                Some(t) => t,
                None => match value {
                    Value::Integer(_, _) => Type::Base(BaseType::Int32),
                    Value::Float(_) => Type::Base(BaseType::Float64),
                    _ => todo!(),
                },
            };

            (IRExpression::Value(value), data_type)
        }
        Expression::Call(mut path, _arguments) => {
            let name = path.components.pop().unwrap();
            let function = types.get_function(current_path, path, &name)?;

            match return_type {
                Some(t) => assert!(t == function.return_type),
                None => {}
            }

            (
                IRExpression::Call(function.name.clone(), Vec::new()),
                function.return_type.clone(),
            )
        }
        Expression::GetVariable(name) => {
            let variable = variables.get(&name)?;
            let data_type = variable.clone().data_type.unwrap();

            (IRExpression::GetVariable(variable.name.clone()), data_type)
        }
        t => todo!("{:?}", t),
    };

    // match return_type {
    //     Some(t) => assert!(t == data_type),
    //     None => {}
    // }

    return Ok((ir, data_type));
}
