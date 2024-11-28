use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    parser::{Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{
    expression::handle_expression, variables::VariablesMap, FileTypes, IRFunction, IRProgram,
    IRType, IRValue, Operation,
};

pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub count: &'a mut NameCounter,
    pub functions: &'a mut Vec<IRFunction>,
    pub types: &'a FileTypes,
}

pub struct FunctionCtx {
    pub variables: VariablesMap,
    pub return_type: Option<Type>,
}

pub fn analyze(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    types: FileTypes,
    program: ParsedProgram,
) -> CompileResult<IRProgram> {
    let mut functions = Vec::new();
    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    let mut ctx = ProgramCtx {
        debug,
        count,
        functions: &mut functions,
        types: &types,
    };

    handle_file(&mut ctx, program.main)?;
    // handle_file(debug, &mut functions, &types, program.main)?;

    return Ok(IRProgram { functions });
}

fn handle_file(program: &mut ProgramCtx, file: ParsedFile) -> CompileResult<()> {
    for (_, import) in file.imports {
        handle_file(program, import)?;
    }

    program
        .debug
        .set_status(format!("Analyzing: {}", file.relative_file_path));

    program.debug.set_path(&file.relative_file_path);

    for info in file.body {
        match info.node {
            Node::Function {
                export: _,
                name: _,
                key,
                parameters,
                return_type,
                body,
            } => {
                let mut variables = VariablesMap::new();
                variables.create_state();

                let mut new_params = Vec::new();
                for (name, data_type) in parameters {
                    let param_key = variables.increment();

                    new_params.push((param_key.clone(), data_type.convert()));

                    variables
                        .insert(&name, false, data_type, info.location.clone())
                        .unwrap();

                    variables.set_key(&name, param_key);
                }

                let missing_return = match body.last() {
                    Some(info) => match info.node {
                        Node::Return(_) => false,
                        _ => !return_type.is_void(),
                    },
                    None => !return_type.is_void(),
                };

                if missing_return {
                    program.debug.error(info.location, "Missing return");
                }

                let ir_type = return_type.convert();
                let mut ctx = FunctionCtx {
                    variables,
                    return_type: Some(return_type),
                };
                let mut operations = Vec::new();

                handle_body(
                    program,
                    &mut ctx,
                    &mut operations,
                    &file.relative_path,
                    body,
                )?;

                ctx.variables.pop_state();

                if match operations.last() {
                    Some(operation) => match operation {
                        Operation::Return {
                            data_type: _,
                            value: _,
                        } => false,
                        _ => true,
                    },
                    None => true,
                } {
                    operations.push(Operation::Return {
                        data_type: IRType::Void,
                        value: IRValue::Null,
                    })
                }

                program.functions.push(IRFunction {
                    name: key,
                    operations,
                    parameters: new_params,
                    return_type: ir_type,
                })
            }
            _ => panic!(),
        }
    }

    return Ok(());
}

fn handle_body(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    operations: &mut Vec<Operation>,
    relative_path: &Path,
    nodes: Vec<NodeInfo>,
) -> CompileResult<()> {
    function.variables.create_state();

    for info in nodes {
        match info.node {
            Node::Call(path, arguments) => {
                let found = match program.types.get_function(relative_path, &path)? {
                    Some(f) => f,
                    None => todo!(),
                };

                function.variables.increment();
                // operations.push(Operation::Call(
                // found.key.clone(),
                // found.return_type.convert(),
                // IRValue::Arguments(Vec::new()),
                // ));

                operations.push(Operation::Call {
                    function: found.key.clone(),
                    return_type: found.return_type.convert(),
                    arguments: IRValue::Arguments(Vec::new()),
                });
                todo!()
            }
            Node::SetVariable { name, expression } => {
                let variable = match function.variables.borrow(&name) {
                    Some(var) => var,
                    None => {
                        program.debug.error(
                            info.location.clone(),
                            format!("Cannot mutate a variable that does not exist '{}'", name),
                        );
                        continue;
                    }
                };

                let (value, data_type) = handle_expression(
                    program,
                    operations,
                    &mut function.variables,
                    relative_path,
                    &variable.data_type.clone(),
                    &info.location,
                    expression,
                )?;

                if !variable.mutable {
                    program.debug.error(
                        info.location.clone(),
                        format!("Cannot mutate unmutable value '{}'", name),
                    );
                }
                
                operations.push(Operation::Store {
                    data_type: data_type.convert(),
                    value,
                    destination: variable.key.clone(),
                });
                
                function.variables.push(name, variable);
            }
            Node::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => {
                let (value, data_type) = handle_expression(
                    program,
                    operations,
                    &mut function.variables,
                    relative_path,
                    &data_type,
                    &info.location,
                    expression,
                )?;

                function
                    .variables
                    .insert(&name, mutable, data_type.clone(), info.location)
                    .unwrap();

                let variable = function.variables.read(&name).unwrap();

                operations.push(Operation::Allocate {
                    destination: variable.key.clone(),
                    data_type: data_type.convert(),
                });

                operations.push(Operation::Store {
                    data_type: data_type.convert(),
                    value,
                    destination: variable.key.clone(),
                });
            }
            Node::Return(expression) => {
                let return_type = &function.return_type;

                let (value, data_type) = handle_expression(
                    program,
                    operations,
                    &mut function.variables,
                    relative_path,
                    return_type,
                    &info.location,
                    expression,
                )?;

                operations.push(Operation::Return {
                    data_type: data_type.convert(),
                    value,
                });
                break;
            }
            _ => todo!("{:#?}", info),
        }
    }

    function.variables.pop_state();
    return Ok(());
}
