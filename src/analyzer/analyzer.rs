use std::collections::HashMap;

use crate::{
    analyzer::types::functions::get_function_types, ASTNode, AnalyzeResult, CompileError,
    Expression, Module, Node, Path, Type, Value,
};

use super::{nodes::IRModule, types::functions::ModuleTypes, variables::Variables};

pub fn analyze(module: Module) -> AnalyzeResult<()> {
    println!("{:#?}", module);

    let types = get_function_types(&module)?;
    println!("{:#?}", types);


    let mut modules = HashMap::new();
    handle_module(module, &types, &mut modules)?;
    // println!("{:#?}", modules);

    todo!()
}

fn handle_module(module: Module, types: &ModuleTypes, modules: &mut HashMap<String, IRModule>) -> AnalyzeResult<()> {
    let ir_module = IRModule::new();


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
                let mut variables = Variables::new();
                handle_scope(types, &mut variables, &parameters, &return_type, body)?;
            }
            _ => continue,
        }
    }



    return Ok(());
}

fn handle_scope(
    types: &ModuleTypes,
    variables: &mut Variables,
    parameters: &Vec<(String, Type)>,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<()> {
    variables.create_state();

    for ast in body {
        match ast.node {
            Node::Scope { is_unsafe, body } => {
                handle_scope(types, variables, parameters, return_type, body)?
            }
            Node::SetVariable(name, expression) => {
                let variable = variables.get(&name)?;
                if variable.mutable == false {
                    return Err(CompileError::new(
                        format!("{:?} is not mutable", name),
                        ast.lines.start,
                    ));
                }
                let expr_type = handle_expression(
                    types,
                    variables,
                    parameters,
                    return_type,
                    &false,
                    expression,
                )?;

                // if variable.data_type.as_ref().unwrap() != expr_type {
                // return Err(CompileError::new(format!("Wrong type {:?}", name), ast.lines.start))
                // };
            }
            Node::DefineVariable {
                mutable,
                name,
                data_type,
                expression,
            } => {
                variables.insert(name.clone(), mutable, data_type)?;
            }
            Node::Return(expression) => {
                let expr = match expression {
                    Some(e) => e,
                    None => continue,
                };
                handle_expression(types, variables, parameters, return_type, &true, expr)?;
            }
            _ => continue,
        }
    }

    variables.pop_state();
    return Ok(());
}

fn handle_expression(
    types: &ModuleTypes,
    variables: &Variables,
    parameters: &Vec<(String, Type)>,
    return_type: &Option<Type>,
    is_return: &bool,
    expression: Expression,
) -> AnalyzeResult<Type> {
    use crate::BaseType::*;

    let var_type: Type = match expression {
        Expression::Value(value) => match value {
            Value::Integer(signed, int) => match return_type {
                Some(t) => {
                    if t.is_integer() {
                        t.clone()
                    } else {
                        return Err(CompileError::new(format!("Wrong return type"), 0));
                    }
                }
                None => Type::Base(Int32),
            },
            Value::Boolean(_) => Type::Base(Boolean),
            v => todo!("{:#?}", v),
        },
        expr => todo!("{:#?}", expr),
    };

    return Ok(var_type);
}
