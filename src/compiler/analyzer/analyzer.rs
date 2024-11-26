use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    parser::{Node, NodeInfo, ParsedFile},
    program::ParsedProgram,
    types::Type,
};

use super::{expression::handle_expression, variables::Variables, FileTypes, IRFunction, IRProgram, Operation};

pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub count: &'a mut NameCounter,
    pub functions: &'a mut Vec<IRFunction>,
    pub types: &'a FileTypes,
}

pub struct FunctionCtx {
    pub variables: Variables,
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
                let mut variables = Variables::new();
                variables.create_state();

                let new_params = Vec::new();
                for (key, data_type) in parameters {
                    variables
                        .insert(&key, false, data_type, info.location.clone())
                        .unwrap();
                }

                let ir_type = return_type.convert();
                let mut ctx = FunctionCtx {
                    variables,
                    return_type: Some(return_type),
                };
                let mut operations = Vec::new();

                handle_body(program, &mut ctx, &mut operations, &file, body)?;
                ctx.variables.pop_state();

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
    nodes: Vec<NodeInfo>,
) -> CompileResult<()> {
    function.variables.create_state();
    for info in nodes {
        match info.node {
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
                    &data_type,
                    expression,
                )?;
                function
                    .variables
                    .insert(&name, mutable, data_type.clone(), info.location)
                    .unwrap();
                let variable = function.variables.get(&name, false).unwrap();

                operations.push(Operation::Allocate(
                    variable.name.clone(),
                    data_type.convert(),
                ));
                operations.push(Operation::Store(
                    data_type.convert(),
                    value,
                    variable.name.clone(),
                ));
            }
            Node::Return(expression) => {
                let return_type = &function.return_type;

                let (value, data_type) = handle_expression(
                    program,
                    operations,
                    &mut function.variables,
                    return_type,
                    expression,
                )?;

                operations.push(Operation::Return(data_type.convert(), value));
            }
            _ => todo!("{:#?}", info),
        }
    }

    function.variables.pop_state();
    return Ok(());
} 
