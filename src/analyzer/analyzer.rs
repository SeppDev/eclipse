use std::collections::HashMap;

use crate::{
    analyzer::types::functions::get_function_types, ASTNode, AnalyzeResult, CompileError,
    Expression, Module, Node, Path, Type, Value,
};

use super::{expression, nodes::IRModule, types::functions::ModuleTypes, variables::Variables};

pub fn analyze(module: Module) -> AnalyzeResult<()> {
    println!("{:#?}", module);

    let types = get_function_types(&module)?;
    println!("{:#?}", types);

    let mut modules = HashMap::new();
    handle_module(module, &types, &mut modules)?;
    // println!("{:#?}", modules);

    todo!()
}

fn handle_module(
    module: Module,
    types: &ModuleTypes,
) -> AnalyzeResult<IRModule> {
    let mut ir_module = IRModule::new();
    63
    for ast in module.body {
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
                let mut variables = Variables::new(parameters);
                handle_scope(types, &mut variables, &return_type, body)?;
            }
            _ => continue,
        }
    }

    for (name, ) in module.submodules {
        submodules.push(handle_module(submodule, types)?);
    }

    return Ok(IRModule {
        submodules,
        body: functions
    });
}

fn handle_scope(
    types: &ModuleTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<()> {
    variables.create_state();

    for ast in body {
        match ast.node {
            Node::Scope { is_unsafe, body } => {
                handle_scope(types, variables, return_type, body)?
            }
            Node::SetVariable(name, expression) => {
                let variable = variables.get(&name)?;
                if variable.mutable == false {
                    return Err(CompileError::new(
                        format!("{:?} is not mutable", name),
                        ast.lines.start,
                    ));
                }
                // let expr_type = handle_expression(
                //     types,
                //     variables,
                //     parameters,
                //     return_type,
                //     &false,
                //     expression,
                // )?;

                // if variable.data_type.as_ref().unwrap() != expr_type {
                // return Err(CompileError::new(format!("Wrong type {:?}", name), ast.lines.start))
                // };
            }
            
            Node::DefineVariable {
                mutable,
                name,
                mut data_type,
                expression,
            } => {
                if data_type.is_none() {
                    data_type = Some(expression::define_variable(
                        types,
                        variables,
                        expression.unwrap(),
                    )?);
                }

                variables.insert(name, mutable, data_type)?;
            }
            Node::Return(expression) => {
                let expr = match expression {
                    Some(e) => e,
                    None => continue,
                };

                todo!();
                break;
                // handle_expression(types, variables, parameters, return_type, &true, expr)?;
            }
            _ => continue,
        }
    }

    variables.pop_state();
    return Ok(());
}
